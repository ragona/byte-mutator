use crate::mutators::bitflipper::BitFlipper;
use crate::reset_buffer::ResetBuffer;

pub mod bitflipper;

/// A logical container for a seed and all of the one-step mutations that it will take
pub struct Strain {}

pub struct MutationSequence {
    pub mutator: Box<dyn Mutator>,
    pub iterations: u32,
    pub range: Range,
}

#[derive(Debug)]
pub enum Range {
    All,
    First(usize),
    Last(usize),
    Range(usize, usize),
}

pub trait Mutator {
    fn mutate(&mut self, bytes: &mut [u8]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flip_bits() {
        let mut bytes = [0u8; 8];
        let mut bf = BitFlipper::new();

        bf.mutate(&mut bytes);

        dbg!(bytes, bf);
    }
}
