//! UndoBuffer is a structure for efficiently undoing changes. It maintains two fixed size
//! ArrayVec structures, exposes interfaces to mutate the write buffer, and then undo methods
//! to restore the buffer to its original state.
//!
//! todo: Variably sized buffers.

use arrayvec::ArrayVec;
use std::cmp::min;
use std::io::Write;

// todo fix -- env var?
const DEFAULT_BUFFER_SIZE: usize = 1024;

#[derive(Debug, Clone)]
/// Fixed size buffer with an original state and a writeable buffer. Tracks which region of the
/// buffer has been exposed for changes and enables an undo of those changes.
pub struct UndoBuffer {
    buffer: ArrayVec<[u8; DEFAULT_BUFFER_SIZE]>,
    original: ArrayVec<[u8; DEFAULT_BUFFER_SIZE]>,
    // todo: Track dirty regions exposed by get_mut or get_mut_range
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

        UndoBuffer { original, buffer }
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.len() == 0
    }

    pub fn get_mut(&mut self) -> &mut [u8] {
        &mut self.buffer[..]
    }

    pub fn get_mut_range(&mut self, start: usize, end: usize) -> &mut [u8] {
        let end = min(self.buffer.len(), end);
        &mut self.buffer[start..end]
    }

    pub fn undo_range(&mut self, start: usize, end: usize) {
        let end = min(self.buffer.len(), end);
        let mut changed = &mut self.buffer[start..end];
        let original = &mut self.original[start..end];

        changed.write_all(&original).expect("Failed to undo range");
    }

    pub fn read(&self) -> &[u8] {
        &self.buffer[..]
    }

    /// Undo all changes and set the readable buffer back to the original state
    pub fn undo_all(&mut self) {
        // note: we need to take a slice of self.buffer here or we write after the existing bytes
        (&mut self.buffer[..])
            .write_all(&self.original[..])
            .expect("Failed to write");
    }
}
