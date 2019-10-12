/// Vec? Just a buffer somewhere?
///
/// 1. bytes
///     - buffer: [u8]
/// 3. mutator sequence
///     - mutators: vec<mutator>
/// 4. mutator
///     - length: SegmentLength, offset maybe?
///     - target: BytesMut (?)
///     - def mutate()
///         returns

use bytes::{BytesMut, BufMut, BigEndian};

enum SegmentLength {
    Fixed(u32),
    Range(u32, u32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut buf = BytesMut::with_capacity(1024);

        let a = buf.();


    }
}
