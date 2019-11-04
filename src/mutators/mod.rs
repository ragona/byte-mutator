//! This module contains the available mutators.
//!
//! # Example
//! ```
//! use byte_mutator::mutators::Mutation;
//! use byte_mutator::mutators::MutatorType::BitFlipper;
//!
//! let bitflipper = Mutation{
//!     range: None,
//!     mutation: BitFlipper {width: 1}
//! };
//!
//! let mut bytes = b"foo".to_vec();
//!
//! bitflipper.mutate(&mut bytes, 0);
//!
//! assert_eq!(&bytes, b"goo");
//! ```

use serde_derive::Deserialize;

use crate::mutators::bitflipper::BitFlipper;

pub mod bitflipper;

#[derive(Debug, Deserialize, Clone)]
pub enum MutatorType {
    BitFlipper { width: u8 },
}

#[derive(Debug, Deserialize, Clone)]
/// A single Mutation, optionally scoped to only operate on a subslice
pub struct Mutation {
    /// Optional subslice range (e.g. Some(0, 3) only mutates the first three bytes)
    pub range: Option<(usize, usize)>,
    /// Type of mutator (e.g. MutatorType::BitFlipper)
    pub mutation: MutatorType,
}

impl Mutation {
    pub fn new(mutator_type: MutatorType, range: Option<(usize, usize)>) -> Mutation {
        Mutation {
            range,
            mutation: mutator_type,
        }
    }
}

/// Maps a MutationType to a specific function call.
impl Mutation {
    /// Execute the mutation
    pub fn mutate(&self, bytes: &mut [u8], i: usize) -> (usize, usize) {
        match self.mutation {
            MutatorType::BitFlipper { width } => BitFlipper::mutate(bytes, i, width),
            // todo: Closure type bitflipper?
        }
    }
}
