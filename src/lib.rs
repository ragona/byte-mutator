//! # Byte Mutator
//!
//! `byte-mutator` is a crate for defining a set of rules by which to mutate byte arrays. It
//! contains two main primitives: `Stage`, and `Mutator`. A `Stage` defines how many iterations
//! to run via the `Iterations` enum, and a `Mutator` defines which `MutatorType` to perform across
//! which range of bytes.
//!
//!`byte-mutator` internally uses an `UndoBuffer`, which is a data structure that exposes mutable
//! `&[u8]` slices, and can undo changes in order to reset and perform another mutation from the
//! clean starting state provided at initialization. This is important to avoid utterly mangling
//! the input; we want to identify small novel changes that produce a different output from the
//! target program, and then reuse that new state to perform further mutations.
//!
//! ```
//! use byte_mutator::*;
//!
//!
//! ```
use serde_derive::Deserialize;

use crate::fuzz_config::FuzzConfig;
use crate::mutators::Mutation;
use crate::undo_buffer::UndoBuffer;

pub mod fuzz_config;
pub mod mutators;
pub mod undo_buffer;

/// Used to limit the number of iterations in a `Stage`.
#[derive(Clone, Debug, Deserialize)]
pub enum Iterations {
    /// One iteration per bit in slice
    Bits,
    /// One iteration per byte in slice
    Bytes,
    /// Goes forever
    Unlimited,
    /// Fixed number of iterations
    Limited(usize),
}

/// Used to define groups of mutations, and how many mutations should be performed.
#[derive(Clone, Debug, Deserialize)]
pub struct Stage {
    /// Current number of iterations.
    /// This can start at > 0 if you want to reproduce something from an earlier run.
    count: usize,
    /// Max number of iterations
    iterations: Iterations,
    /// Group of mutations, all of which are performed every time.
    mutations: Vec<Mutation>,
}

impl Stage {
    pub fn new(count: usize, mutations: Vec<Mutation>, iterations: Iterations) -> Self {
        Self {
            count,
            mutations,
            iterations,
        }
    }

    pub fn is_done(&self, num_bytes: usize) -> bool {
        match self.iterations {
            Iterations::Bits => self.count >= num_bytes * 8,
            Iterations::Bytes => self.count >= num_bytes,
            Iterations::Limited(n) => self.count >= n,
            Iterations::Unlimited => false,
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
        Stage::new(0, vec![], Iterations::Unlimited)
    }
}

#[derive(Debug, Clone)]
pub struct ByteMutator {
    bytes: UndoBuffer,
    stages: Vec<Stage>,
    cur_stage: usize,
}

impl ByteMutator {
    /// Create a new `ByteMutator` from the provided byte slice. Copies into two internal buffers.
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            bytes: UndoBuffer::new(bytes),
            stages: vec![],
            cur_stage: 0,
        }
    }

    /// Creates a new `ByteMutator` and consumes the `stages` configured in `config`
    pub fn new_from_config(bytes: &[u8], config: FuzzConfig) -> Self {
        Self {
            bytes: UndoBuffer::new(bytes),
            stages: config.stages,
            cur_stage: 0,
        }
    }

    /// Number of outstanding stages
    pub fn remaining_stages(&self) -> usize {
        self.stages.len()
    }

    /// Add a stage
    pub fn add_stage(&mut self, stage: Stage) {
        self.stages.push(stage);
    }

    /// Advance the mutation one step. Resets outstanding changes, advances the stage state,
    /// and mutates using all mutators defined in the stage.
    pub fn next(&mut self) {
        let stage = match self.stages.get_mut(0) {
            None => return, // nothing to do
            Some(s) => s,
        };

        // we reset the last change first so that we're getting small changes not huge ones
        self.bytes.undo();

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
            self.bytes.undo();
        }
    }

    /// Returns an immutable slice of the mutable buffer.
    /// Exposes the current state of the `ByteMutator`.
    pub fn read(&self) -> &[u8] {
        self.bytes.read()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mutators::MutatorType;

    #[test]
    fn mutator_stage() {
        let mut byte_mutator = ByteMutator::new(b"foo");

        byte_mutator.add_stage(Stage::new(
            0,
            vec![Mutation {
                range: None,
                mutation: MutatorType::BitFlipper { width: 1 },
            }],
            Iterations::Limited(10),
        ));

        assert_eq!(byte_mutator.remaining_stages(), 1);

        for _ in 0..10 {
            byte_mutator.next();
        }

        assert_eq!(byte_mutator.remaining_stages(), 0);
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
