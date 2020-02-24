//! This module contains the available mutators.
//!
//! # Example
//! ```
//! use byte_mutator::mutators::Mutation;
//! use byte_mutator::mutators::MutationType::BitFlipper;
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
pub enum MutationType {
    /// Flips `width` number of bits.
    BitFlipper { width: u8 },
}

#[derive(Debug, Deserialize, Clone)]
/// A single mutation, optionally scoped to only operate on a subslice
pub struct Mutation {
    /// Optional subslice range (e.g. Some((0, 3)) only mutates the first three bytes)
    pub range: Option<(usize, usize)>,
    /// Type of mutator (e.g. MutatorType::BitFlipper)
    pub mutation: MutationType,
}

impl Mutation {
    /// Create a new `Mutation`, optionally scoped to operate only on a subslice
    pub const fn new(mutator_type: MutationType, range: Option<(usize, usize)>) -> Self {
        Self {
            range,
            mutation: mutator_type,
        }
    }

    /// Execute the mutation
    pub fn mutate(&self, bytes: &mut [u8], i: usize) {
        match self.mutation {
            MutationType::BitFlipper { width } => BitFlipper::mutate(bytes, i, width),
            // todo: Closure type bitflipper?
        }
    }
}
