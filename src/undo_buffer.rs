use arrayvec::ArrayVec;
use std::cmp::min;
use std::hash::Hasher;
use std::io::Write;

// todo fix -- env var?
const DEFAULT_BUFFER_SIZE: usize = 1024;

#[derive(Debug, Clone)]
pub struct UndoBuffer {
    buffer: ArrayVec<[u8; DEFAULT_BUFFER_SIZE]>,
    original: ArrayVec<[u8; DEFAULT_BUFFER_SIZE]>,
}

impl UndoBuffer {
    pub fn new(buf: &[u8]) -> UndoBuffer {
        let mut original = ArrayVec::<[u8; DEFAULT_BUFFER_SIZE]>::new();
        let mut buffer = ArrayVec::<[u8; DEFAULT_BUFFER_SIZE]>::new();

        // todo: will panic if buf.len() > original.len()
        (&mut original).write(buf);
        (&mut buffer).write(buf);

        UndoBuffer { original, buffer }
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn as_mut(&mut self) -> &mut [u8] {
        &mut self.buffer[..]
    }

    pub fn get_mut_range(&mut self, start: usize, end: usize) -> &mut [u8] {
        let end = min(self.buffer.len(), end);
        &mut self.buffer[start..end]
    }

    pub fn undo_range(&mut self, start: usize, end: usize) {
        let mut changed = &mut self.buffer[start..end];
        let mut original = &mut self.original[start..end];

        changed.write(&original);
    }

    pub fn read(&self) -> &[u8] {
        &self.buffer[..]
    }

    pub fn undo_all(&mut self) {
        // note: we need to take a slice of self.buffer here or we write after the existing bytes
        (&mut self.buffer[..]).write(&self.original[..]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //    #[test]
    //    fn foo() {
    //         todo:
    //    }
}
