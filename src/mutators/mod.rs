use crate::mutators::bitflipper::BitFlipper;

pub mod bitflipper;

#[derive(Debug)]
pub enum Range {
    All,
    First(usize),
    Last(usize),
    Range(usize, usize),
}

pub trait Mutate {
    fn mutate(&mut self, bytes: &mut [u8]);
    fn is_done(&mut self, bytes: &mut [u8]) -> bool;
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
