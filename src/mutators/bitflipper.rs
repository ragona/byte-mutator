#[derive(Debug, Copy, Clone)]
pub struct BitFlipper {}

impl BitFlipper {
    pub fn mutate(bytes: &mut [u8], i: usize, width: u8) -> (usize, usize) {
        let i = i % (bytes.len() * 8);
        let byte = i / 8;
        let bit = i % 8;
        let v: u8 = bytes[byte] ^ 1 << bit as u8;

        // todo: Implement width
        bytes[byte] = v;

        (i, i + width as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::undo_buffer::UndoBuffer;

    #[test]
    fn flip_bit() {
        let mut buffer = UndoBuffer::new(b"foo");

        // first bit should flip resulting in 'goo'
        // 0b1100110 -> 0b1100111, 103 -> 102, f -> g
        BitFlipper::mutate(buffer.get_mut(), 0, 1);
        assert_eq!(buffer.read(), b"goo");
    }
}
