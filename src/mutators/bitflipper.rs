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

impl Mutate for BitFlipper {
    fn mutate(&mut self, bytes: &mut [u8]) {
        let i = match self.range {
            Range::All => self.count % bytes.len(),
            Range::First(n) => self.count % n,
            Range::Last(n) => self.count % n,
            Range::Range(start, end) => self.count % (end - start),
        };

        bytes[i] = 1;

        self.count += 1;
    }

    fn is_done(&mut self, bytes: &mut [u8]) -> bool {
        match self.range {
            Range::All => self.count >= bytes.len(),
            Range::First(n) => self.count >= n,
            Range::Last(n) => self.count >= n,
            Range::Range(start, end) => self.count >= end - start,
        }
    }
}
