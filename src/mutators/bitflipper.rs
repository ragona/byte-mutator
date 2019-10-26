use super::*;

#[derive(Debug)]
pub struct BitFlipper {
    pub width: u8,
    pub range: Range,
    count: usize,
}

impl BitFlipper {
    pub fn new() -> BitFlipper {
        BitFlipper {
            width: 1,
            count: 0,
            range: Range::All,
        }
    }
}

impl Mutator for BitFlipper {
    fn mutate(&mut self, bytes: &mut [u8]) {
        let i = match self.range {
            Range::All => self.count % bytes.len(),
            Range::First(n) => self.count % n,
            Range::Last(n) => self.count % n,
            Range::Range(start, end) => self.count % (end - start),
        };

        let byte = i / 8;
        let bit = i % 8;
        let v: u8 = bytes[byte] ^ 1 << bit as u8;

        bytes[i] = v;

        self.count += 1;
    }
}
