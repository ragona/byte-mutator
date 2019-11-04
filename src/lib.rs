use serde_derive::Deserialize;

use crate::fuzz_config::FuzzConfig;
use crate::mutators::Mutation;
use crate::undo_buffer::UndoBuffer;
use crate::Iterations::Unlimited;

pub mod fuzz_config;
pub mod mutators;
pub mod undo_buffer;

#[derive(Clone, Debug, Deserialize)]
pub enum Iterations {
    Bits,
    Bytes,
    Unlimited,
    Limited(usize),
}

#[derive(Clone, Debug, Deserialize)]
pub struct Stage {
    count: usize,
    iterations: Iterations,
    mutations: Vec<Mutation>,
}

impl Stage {
    pub fn new() -> Stage {
        Stage {
            count: 0,
            mutations: vec![],
            iterations: Unlimited,
        }
    }

    pub fn limited(limit: usize) -> Stage {
        Stage {
            count: 0,
            mutations: vec![],
            iterations: Iterations::Limited(limit),
        }
    }

    pub fn is_done(&self, num_bytes: usize) -> bool {
        match self.iterations {
            Iterations::Bits => self.count >= num_bytes * 8,
            Iterations::Bytes => self.count >= num_bytes,
            Iterations::Limited(n) => self.count >= n,
            Unlimited => false,
        }
    }

    pub fn next(&mut self) {
        self.count += 1;
    }

    pub fn add_mutation(&mut self, mutation: Mutation) {
        self.mutations.push(mutation);
    }
}

impl Default for Stage {
    fn default() -> Self {
        Stage::new()
    }
}

#[derive(Debug, Clone)]
pub struct ByteMutator {
    bytes: UndoBuffer,
    stages: Vec<Stage>,
    cur_stage: usize,
}

impl ByteMutator {
    pub fn new(bytes: &[u8]) -> ByteMutator {
        ByteMutator {
            bytes: UndoBuffer::new(bytes),
            stages: vec![],
            cur_stage: 0,
        }
    }

    pub fn new_from_config(bytes: &[u8], config: FuzzConfig) -> ByteMutator {
        ByteMutator {
            bytes: UndoBuffer::new(bytes),
            stages: config.stages,
            cur_stage: 0,
        }
    }

    pub fn remaining_stages(&self) -> usize {
        self.stages.len()
    }

    pub fn add_stage(&mut self, stage: Stage) {
        self.stages.push(stage);
    }

    pub fn next(&mut self) {
        // nothing to do
        if self.stages.is_empty() {
            return;
        }

        // todo: ranged undo
        // we reset the last change first so that we're getting small changes not huge ones
        self.bytes.undo_all();

        let stage = &mut self.stages[0];
        for mutation in &mut stage.mutations {
            match mutation.range {
                Some((start, end)) => {
                    mutation.mutate(self.bytes.get_mut_range(start, end), stage.count)
                }
                None => mutation.mutate(self.bytes.get_mut(), stage.count),
            };
        }

        stage.next();

        if stage.is_done(self.bytes.len()) {
            self.stages.drain(..1); // todo: Is this right?
            self.bytes.undo_all();
        }
    }

    pub fn read(&self) -> &[u8] {
        self.bytes.read()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mutators::bitflipper::BitFlipper;
    use crate::mutators::MutatorType;

    #[test]
    fn mutate_and_reset() {
        let mut buffer = UndoBuffer::new(b"foo");

        // first bit should flip resulting in 'goo'
        // 0b1100110 -> 0b1100111, 103 -> 102, f -> g
        BitFlipper::mutate(buffer.get_mut(), 0, 1);
        assert_eq!(buffer.read(), b"goo");

        // should be back to 'foo'
        buffer.undo_all();
        assert_eq!(buffer.read(), b"foo");
    }

    #[test]
    fn mutate_reset_range() {
        // clamp changes to the last byte
        let (min, max) = (2, 3);
        let mut buffer = undo_buffer::UndoBuffer::new(b"foo");
        let range = buffer.get_mut_range(min, max);

        // flip a bit
        let (start, end) = BitFlipper::mutate(range, 0, 1);

        // assert that something changed
        assert_ne!(buffer.read()[0..3], b"foo"[..]);

        // set it back
        buffer.undo_range(min + start, min + end);

        // make sure we match
        assert_eq!(buffer.read()[0..3], b"foo"[..]);
    }

    #[test]
    fn mutator_stage() {
        let mut byte_mutator = ByteMutator::new(b"foo");
        let mut stage = Stage::limited(10);

        stage.add_mutation(Mutation::new(MutatorType::BitFlipper { width: 1 }, None));
        byte_mutator.add_stage(stage);

        for _ in 0..20 {
            byte_mutator.next();
        }
    }
    #[test]
    fn mutator_from_config() {
        let mut mutator = ByteMutator::new_from_config(b"foo", FuzzConfig::default());

        for _ in 0..20 {
            mutator.next();
        }

        assert!(mutator.remaining_stages() >= 1);
    }
}
