use crate::mutators::{MutationSequence, Mutator, Range};
use crate::reset_buffer::ResetBuffer;

use arrayvec;

pub mod mutators;
pub mod reset_buffer;
pub mod undo_buffer;

pub type RangeLimit = (usize, usize);

pub struct ByteMutator {
    bytes: ResetBuffer,
    mutators: Vec<Box<dyn Mutator>>,
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
        self.mutators[0].mutate(self.bytes.as_mut());
    }

    pub fn read(&self) -> &[u8] {
        self.bytes.read()
    }

    pub fn add_mutator(&mut self, mutator: Box<dyn Mutator>) {
        self.mutators.push(mutator);
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
        let mut mutator = BitFlipper::new(1);

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

        foo.add_mutator(Box::new(BitFlipper::new(1)));
        foo.next();

        dbg!(foo.read());
    }

    #[test]
    fn wtf() {
        // clamp changes to the last byte
        let (min, max) = (2, 3);
        let mut buffer = undo_buffer::UndoBuffer::new(b"foo");
        let mut mutator = BitFlipper::new(1);
        let mut range = buffer.get_mut_range(min, max);

        // flip a bit
        let (start, end) = mutator.mutate(range);

        // assert that something changed
        assert_ne!(buffer.buffer[0..3], b"foo"[..]);

        // set it back
        buffer.undo_range(min + start, min + end);

        // make sure we match
        assert_eq!(buffer.buffer[0..3], b"foo"[..]);
    }
}
