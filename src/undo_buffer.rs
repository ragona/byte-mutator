use arrayvec::ArrayVec;
use std::io::Write;

// todo fix -- env var?
const BUFFER_SIZE: usize = 16;

pub struct UndoBuffer {
    pub buffer: ArrayVec<[u8; BUFFER_SIZE]>,
    original: ArrayVec<[u8; BUFFER_SIZE]>,
}

impl UndoBuffer {
    pub fn new(buf: &[u8]) -> UndoBuffer {
        let mut original = ArrayVec::<[u8; BUFFER_SIZE]>::new();
        let mut buffer = ArrayVec::<[u8; BUFFER_SIZE]>::new();

        // todo: will panic if buf.len() > original.len()
        (&mut original).write(buf);
        (&mut buffer).write(buf);

        UndoBuffer { original, buffer }
    }

    pub fn get_mut_range(&mut self, start: usize, end: usize) -> &mut [u8] {
        &mut self.buffer[start..end]
    }

    pub fn undo_range(&mut self, start: usize, end: usize) {
        let mut changed = &mut self.buffer[start..end];
        let mut original = &mut self.original[start..end];

        changed.write(&original);
    }
}
