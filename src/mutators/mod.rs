use crate::mutators::bitflipper::BitFlipper;
use crate::ByteMutator;

pub mod bitflipper;

/// A logical container for a seed and all of the one-step mutations that it will take
pub struct Strain {}

pub struct MutationSequence {
    pub mutator: Box<dyn Mutator>,
    pub iterations: u32,
}

pub type Range = (usize, usize);

pub trait Mutator {
    /// Takes a mutable reference to a slice, and performs some operation, such as flipping bits,
    /// incrementing or decrementing numbers. Returns a tuple representing the index of the first
    /// and last bytes modified, used to undo the range that was mutated.
    fn mutate(&mut self, bytes: &mut [u8]) -> Range;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flip_bits() {
        let mut bytes = [0u8; 8];
        let mut bf = BitFlipper::new(1);

        bf.mutate(&mut bytes);

        dbg!(bytes, bf);
    }
}
