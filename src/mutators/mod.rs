use crate::mutators::bitflipper::BitFlipper;

pub mod bitflipper;

pub enum Range {
    All,
    First(usize),
    Last(usize),
    Range(usize, usize),
}

pub trait Mutate {
    fn mutate(&mut self, bytes: &mut [u8], range: Range);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flip_bits() {
        let mut bytes = [0u8; 8];
        let mut bf = BitFlipper::new(1);

        bf.mutate(&mut bytes, Range::All);

        dbg!(bytes, bf);
    }
}
