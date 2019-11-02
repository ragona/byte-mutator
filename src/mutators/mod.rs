use crate::mutators::bitflipper::BitFlipper;
use crate::ByteMutator;

pub mod bitflipper;

pub trait Mutator {
    /// Takes a mutable reference to a slice, and performs some operation, such as flipping bits,
    /// incrementing or decrementing numbers. Returns a tuple representing the index of the first
    /// and last bytes modified, used to undo the range that was mutated.
    fn mutate(&mut self, bytes: &mut [u8]) -> (usize, usize);
}

pub struct MutatorFactory {}

impl MutatorFactory {
    pub fn from_type(t: MutatorType) -> Box<dyn Mutator> {
        match t {
            MutatorType::BitFlipper { width } => Box::new(BitFlipper::new(width)),
        }
    }
}

pub enum MutatorType {
    BitFlipper { width: u8 },
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
