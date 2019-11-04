use arrayvec;

use crate::fuzz_config::FuzzConfig;
use crate::mutators::{Mutation, MutatorType};
use crate::undo_buffer::UndoBuffer;
use serde_derive::Deserialize;
use std::cmp::max;

pub mod fuzz_config;
pub mod mutators;
pub mod undo_buffer;

#[derive(Debug, Deserialize, Clone)]
pub struct Stage {
    cur_iterations: usize,
    max_iterations: usize,
    mutations: Vec<Mutation>,
}

impl Stage {
    pub fn new(max_iterations: usize) -> Stage {
        Stage {
            max_iterations,
            cur_iterations: 0,
            mutations: vec![],
        }
    }

    pub fn is_done(&self) -> bool {
        match self.max_iterations {
            // no figured max means go forever
            0 => false,
            _ => self.cur_iterations >= self.max_iterations,
        }
    }

    pub fn next(&mut self) {
        self.cur_iterations += 1;
    }

    pub fn add_mutation(&mut self, mutation: Mutation) {
        self.mutations.push(mutation);
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
        // todo: ranged undo
        // we reset the last change first so that we're getting small changes not huge ones
        self.bytes.undo_all();

        // nothing to do
        if self.stages.len() == 0 {
            return;
        }

        let stage = &mut self.stages[0];
        for mutation in &mut stage.mutations {
            match mutation.range {
                Some((start, end)) => {
                    mutation.mutate(self.bytes.get_mut_range(start, end), stage.cur_iterations)
                }
                None => mutation.mutate(self.bytes.as_mut(), stage.cur_iterations),
            };
        }

        stage.next();

        if stage.is_done() {
            self.stages.drain(..1); // todo: Is this right?
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

    #[test]
    fn mutate_and_reset() {
        let mut buffer = UndoBuffer::new(b"foo");

        // first bit should flip resulting in 'goo'
        // 0b1100110 -> 0b1100111, 103 -> 102, f -> g
        BitFlipper::mutate(buffer.as_mut(), 0, 1);
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
        let mut range = buffer.get_mut_range(min, max);

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
        let mut stage = Stage::new(10);

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
