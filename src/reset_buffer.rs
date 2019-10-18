use std::io::Write;

// todo: Varying sizes
const BUFFER_SIZE: usize = 1024;

/// Fixed size buffer that can be reset to an original state
pub struct ResetBuffer {
    /// Editable buffer
    pub buffer: [u8; BUFFER_SIZE],
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
    pub fn seed(&mut self, buffer: &[u8]) {
        self.end = buffer.len();
        byte_copy(&buffer, &mut self.seed);
    }

    /// Restores self.buffer to its original state, discarding changes
    pub fn reset(&mut self) {
        byte_copy(&self.seed, &mut self.buffer);
    }
}

fn byte_copy(from: &[u8], mut to: &mut [u8]) -> usize {
    to.write(from).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        let x = ResetBuffer::new();
    }
}
