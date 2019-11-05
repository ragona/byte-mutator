//! `UndoBuffer` is a structure for efficiently undoing changes. It maintains two fixed size
//! `ArrayVec` structures, exposes interfaces to mutate the write buffer, and then undo methods
//! to restore the buffer to its original state.
//!
//! todo: Variably sized buffers.

use arrayvec::ArrayVec;
use std::cmp::{max, min};
use std::io::Write;

// todo fix -- env var?
const DEFAULT_BUFFER_SIZE: usize = 1024;

#[derive(Debug, Clone)]
/// Fixed size buffer with an original state and a writeable buffer. Tracks which region of the
/// buffer has been exposed for changes and enables an undo of those changes.
pub struct UndoBuffer {
    buffer: ArrayVec<[u8; DEFAULT_BUFFER_SIZE]>,
    original: ArrayVec<[u8; DEFAULT_BUFFER_SIZE]>,
    dirty: Option<(usize, usize)>,
}

impl UndoBuffer {
    pub fn new(buf: &[u8]) -> UndoBuffer {
        let mut original = ArrayVec::<[u8; DEFAULT_BUFFER_SIZE]>::new();
        let mut buffer = ArrayVec::<[u8; DEFAULT_BUFFER_SIZE]>::new();

        // todo: will panic if buf.len() > original.len()
        (&mut original)
            .write_all(buf)
            .expect("Failed to copy into UndoBuffer");
        (&mut buffer)
            .write_all(buf)
            .expect("Failed to copy into UndoBuffer");

        UndoBuffer {
            original,
            buffer,
            dirty: None,
        }
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.len() == 0
    }

    pub fn get_mut(&mut self) -> &mut [u8] {
        self.dirty = Some((0, self.buffer.len()));
        &mut self.buffer[..]
    }

    pub fn get_mut_range(&mut self, start: usize, end: usize) -> &mut [u8] {
        // protect against running off the end of the buffer
        let end = min(self.buffer.len(), end);
        self.dirty = match self.dirty {
            Some(range) => {
                // expand to cover range
                Some((min(range.0, start), max(range.1, end)))
            }
            None => Some((start, end)),
        };

        &mut self.buffer[start..end]
    }

    pub fn read(&self) -> &[u8] {
        &self.buffer[..]
    }

    /// Undo all changes and set the readable buffer back to the original state
    pub fn undo(&mut self) {
        let (start, end) = match self.dirty {
            None => {
                return; // no-op
            }
            Some(range) => range,
        };

        (&mut self.buffer[start..end])
            .write_all(&self.original[start..end])
            .expect("Failed to write");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mutators::bitflipper::BitFlipper;

    #[test]
    fn mutate_and_reset() {
        let mut buffer = UndoBuffer::new(b"foo");

        // first bit should flip resulting in 'goo'
        // 0b1100110 -> 0b1100111, 103 -> 102, f -> g
        BitFlipper::mutate(buffer.get_mut(), 0, 1);
        assert_eq!(buffer.read(), b"goo");

        // should be back to 'foo'
        buffer.undo();
        assert_eq!(buffer.read(), b"foo");
    }

    #[test]
    fn mutate_reset_range() {
        // clamp changes to the last byte
        let (min, max) = (2, 3);
        let mut buffer = UndoBuffer::new(b"foo");
        let range = buffer.get_mut_range(min, max);

        // flip a bit
        BitFlipper::mutate(range, 0, 1);

        // assert that something changed
        assert_ne!(buffer.read()[0..3], b"foo"[..]);

        // set it back
        buffer.undo();

        // make sure we match
        assert_eq!(buffer.read()[0..3], b"foo"[..]);
    }
}
