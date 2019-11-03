use crate::mutators::bitflipper::BitFlipper;
use crate::ByteMutator;

pub mod bitflipper;

#[derive(Debug)]
pub enum MutatorType {
    BitFlipper { width: u8 },
}

pub struct Mutation {
    pub range: Option<(usize, usize)>,
    inner: MutatorType,
}

impl Mutation {
    pub fn new(mutator_type: MutatorType, range: Option<(usize, usize)>) -> Mutation {
        Mutation {
            range,
            inner: mutator_type,
        }
    }
}

impl Mutation {
    pub fn mutate(&mut self, bytes: &mut [u8], i: usize) -> (usize, usize) {
        match self.inner {
            MutatorType::BitFlipper { width } => BitFlipper::mutate(bytes, i, width),
        }
    }
}
