#![feature(test)]

extern crate test;

use crate::mutators::{MutationSequence, Mutator, Range};
use crate::reset_buffer::ResetBuffer;

pub mod mutators;
pub mod reset_buffer;

pub struct ByteMutator {
    bytes: ResetBuffer,
    mutators: Vec<MutationSequence>,
}

impl ByteMutator {
    pub fn new(bytes: &[u8]) -> ByteMutator {
        ByteMutator {
            bytes: ResetBuffer::from_seed(bytes),
            mutators: vec![],
        }
    }

    pub fn next(&mut self) {
        // set cur mutator
        // we reset first so that we're getting small changes not huge ones
        self.bytes.reset();
        unimplemented!();
        //self.mutators[0].mutate(self.bytes.as_mut())
    }

    pub fn read(&self) -> &[u8] {
        self.bytes.read()
    }

    pub fn add_mutation(&mut self, sequence: MutationSequence) {
        self.mutators.push(sequence);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mutators::bitflipper::BitFlipper;
    use crate::reset_buffer::ResetBuffer;

    #[test]
    fn mutate_and_reset() {
        let mut buffer = ResetBuffer::new();
        let mut mutator = BitFlipper::new();

        buffer.seed(b"foo").unwrap();
        mutator.mutate(buffer.as_mut());

        // first bit should flip resulting in 'goo'
        // 0b1100110 -> 0b1100111, 103 -> 102, f -> g
        assert_eq!(buffer.read(), b"goo");

        buffer.reset().unwrap();

        // should be back to 'foo'
        assert_eq!(buffer.read(), b"foo");
    }

    #[test]
    fn mutator_list() {
        let mut foo = ByteMutator::new(b"foo");

        foo.add_mutation(MutationSequence {
            mutator: Box::new(BitFlipper::new()),
            iterations: 0,
            range: Range::All,
        });

        foo.next();

        dbg!(foo.read());
    }
}
