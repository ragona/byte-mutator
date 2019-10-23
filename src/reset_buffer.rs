use std::io::{self, Error, Write};

// todo: Varying sizes
const BUFFER_SIZE: usize = 8;

/// Fixed size buffer that can be reset to an original state
pub struct ResetBuffer {
    /// Editable buffer
    buffer: [u8; BUFFER_SIZE],
    /// Stable state of the buffer; does not change
    seed: [u8; BUFFER_SIZE],
    /// End of the used data
    end: usize,
}

impl ResetBuffer {
    pub fn new() -> ResetBuffer {
        ResetBuffer {
            buffer: [0; BUFFER_SIZE],
            seed: [0; BUFFER_SIZE],
            end: 0,
        }
    }

    /// Sets the default state of the buffer
    pub fn seed(&mut self, buffer: &[u8]) -> io::Result<usize> {
        self.buffer.as_mut().write(buffer)?;
        self.seed.as_mut().write(buffer)?;
        self.end = buffer.len();

        Ok(buffer.len())
    }

    /// Restores self.buffer to its original state, discarding changes
    pub fn reset(&mut self) -> io::Result<usize> {
        self.buffer.as_mut().write(&self.seed)
    }

    pub fn read(&self) -> &[u8] {
        &self.buffer[..self.end]
    }
}

impl Write for ResetBuffer {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        self.buffer.as_mut().write(buf)
    }

    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed() {
        let mut buffer = ResetBuffer::new();

        buffer.seed(b"hello").unwrap();

        assert_eq!(buffer.read(), b"hello");
    }

    #[test]
    fn reset() {
        let mut buffer = ResetBuffer::new();

        buffer.seed(b"hello").unwrap();
        buffer.write(b"foo").unwrap();
        buffer.reset().unwrap();

        assert_eq!(buffer.read(), b"hello");
    }
}
