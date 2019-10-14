use super::*;

#[derive(Debug)]
pub struct BitFlipper {
    pub width: u8,
    count: u32,
}

impl BitFlipper {
    pub fn new(width: u8) -> BitFlipper {
        BitFlipper { width, count: 0 }
    }
}

impl Mutate for BitFlipper {
    fn mutate(&mut self, bytes: &mut [u8], range: Range) {
        self.count += 1;
        bytes[0] = 1;
    }
}
