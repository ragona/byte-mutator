//! # Byte Mutator
//!
//! `byte-mutator` is a crate for defining a set of rules by which to mutate byte arrays. It
//! contains two main primitives: `Stage`, and `Mutator`. A `Stage` allows multiple mutations per
//! step, and a Mutator is a small stateful object that
//!
//! ```
//! ```
use serde_derive::Deserialize;

pub use crate::fuzz_config::FuzzConfig;
pub use crate::mutators::{Mutation, MutationType};
pub use crate::undo_buffer::UndoBuffer;

pub mod fuzz_config;
pub mod mutators;
pub mod undo_buffer;

/// Used to limit the number of iterations in a `Stage`.
/// todo: Unused, fix
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
    pub count: usize,
    /// Optional max number of iterations
    pub max: Option<usize>,
    /// Group of mutations, all of which are performed every time.
    pub mutations: Vec<Mutation>,
}

impl Stage {
    pub fn new(count: usize, mutations: Vec<Mutation>, max: Option<usize>) -> Self {
        Self {
            count,
            mutations,
            max,
        }
    }

    /// Returns whether the stage is complete
    pub fn is_done(&self) -> bool {
        match self.max {
            None => false,
            Some(n) => self.count >= n,
        }
    }

    /// Advances the internal state of the `Stage`
    pub fn next(&mut self) {
        self.count += 1;
    }

    /// Add a mutation
    pub fn add_mutation(&mut self, mutation: Mutation) {
        self.mutations.push(mutation);
    }
}

impl Default for Stage {
    /// Default `Stage` with no mutations and unlimited iterations
    fn default() -> Self {
        Stage::new(0, vec![], None)
    }
}

/// A defined set of stages of mutations
#[derive(Debug, Clone)]
pub struct ByteMutator {
    /// Queue of outstanding stages, ordered from first to last. Drains from the front.
    stages: Vec<Stage>,
    /// The in-progress stage.
    cur_stage: usize,
}

impl ByteMutator {
    /// Create a new `ByteMutator`.
    pub fn new() -> Self {
        Self {
            stages: vec![],
            cur_stage: 0,
        }
    }

    pub fn with_stages(mut self, stages: Vec<Stage>) -> Self {
        self.stages = stages;
        self
    }

    /// Creates a new `ByteMutator` and consumes the `stages` configured in `config`
    pub fn new_from_config(config: FuzzConfig) -> Self {
        Self {
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
    /// todo: Make this an actual iterator
    pub fn next(&mut self) {
        let stage = match self.stages.get_mut(0) {
            None => return, // nothing to do
            Some(s) => s,
        };

        stage.next();

        if stage.is_done() {
            self.stages.drain(..1);
        }
    }

    pub fn mutate(&self, bytes: &mut [u8]) {
        let stage = match self.stages.get(0) {
            None => return, // nothing to do
            Some(s) => s,
        };

        for mutation in &stage.mutations {
            match mutation.range {
                Some((start, end)) => mutation.mutate(&mut bytes[start..end], stage.count),
                None => mutation.mutate(bytes, stage.count),
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mutators::MutationType;

    #[test]
    fn mutator_stage() {
        let mut byte_mutator = ByteMutator::new();

        byte_mutator.add_stage(Stage::new(
            0,
            vec![Mutation {
                range: None,
                mutation: MutationType::BitFlipper { width: 1 },
            }],
            Some(10),
        ));

        assert_eq!(byte_mutator.remaining_stages(), 1);

        for _ in 0..10 {
            byte_mutator.next();
        }

        assert_eq!(byte_mutator.remaining_stages(), 0);
    }
}
