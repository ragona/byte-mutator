use crate::mutators::bitflipper::BitFlipper;
use crate::ByteMutator;
use serde_derive::Deserialize;
pub mod bitflipper;

#[derive(Debug, Deserialize, Clone)]
pub enum MutatorType {
    BitFlipper { width: u8 },
}

#[derive(Debug, Deserialize, Clone)]
pub struct Mutation {
    pub range: Option<(usize, usize)>,
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

impl Mutation {
    pub fn mutate(&mut self, bytes: &mut [u8], i: usize) -> (usize, usize) {
        match self.mutation {
            MutatorType::BitFlipper { width } => BitFlipper::mutate(bytes, i, width),
        }
    }
}
