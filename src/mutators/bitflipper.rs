use super::*;

#[derive(Debug)]
pub struct BitFlipper {
    pub width: u8,
    pub count: usize,
}

impl BitFlipper {
    pub fn new(width: u8) -> BitFlipper {
        BitFlipper { width, count: 0 }
    }
}

impl Mutator for BitFlipper {
    fn mutate(&mut self, bytes: &mut [u8]) -> (usize, usize) {
        let i = self.count % bytes.len();
        let byte = i / 8;
        let bit = i % 8;
        let v: u8 = bytes[byte] ^ 1 << bit as u8;

        // todo: Implement width
        bytes[i] = v;
        self.count += 1;

        (i, i + self.width as usize)
    }
}
