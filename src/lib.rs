#![feature(test)]

extern crate test;

use crate::mutators::Mutator;

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
///
///
pub mod mutators;
pub mod reset_buffer;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mutators::bitflipper::BitFlipper;
    use crate::mutators::{Mutator, Range};
    use crate::reset_buffer::ResetBuffer;
    use std::io::Write;
    use test::Bencher;

    fn mutate_buffer() {
        let mut buffer = b"foo".to_vec();
        let mut mutator = BitFlipper::new();

        mutator.mutate(&mut buffer);
        mutator.mutate(&mut buffer);
        mutator.mutate(&mut buffer);

        assert_eq!(buffer, [103, 101, 99]);
    }

    #[test]
    fn checksum_example() {
        // Let's imagine we have a simple web application that we want
        // to fuzz. Its payload has a couple of fields and a checksum:
        //
        // action: transfer
        // from: alice
        // to: bob
        // checksum: 6cc49303d213f798967ce815ad59ad3c
        //
        let mut seed = b"action: transfer\nfrom:alice \nto: bob\n";

        // range: First(len(msg) - len(checksum))
        // normal fuzzing of a defined range of the payload

        // range:

        // add another mutator that calculates a valid checksum
    }

    #[bench]
    fn bench_copy_buffer(b: &mut Bencher) {
        let mut buffer_a = [0u8; 1024];
        let mut buffer_b = [1u8; 1024];
        b.iter(|| {
            for i in 0..1024 {
                buffer_a[i] = buffer_b[i]
            }
        });
    }

    fn byte_copy(from: &[u8], mut to: &mut [u8]) -> usize {
        to.write(from).unwrap()
    }

    #[bench]
    fn bench_copy_buffer_2(b: &mut Bencher) {
        let mut buffer_a = [0u8; 1024];
        let mut buffer_b = [1u8; 1024];
        b.iter(|| byte_copy(&mut buffer_a, &mut buffer_b));
    }

    #[bench]
    fn bench_copy_vec(b: &mut Bencher) {
        let mut vec_a: Vec<u8> = vec![0; 1024];
        let mut vec_b: Vec<u8> = vec![1; 1024];
        b.iter(|| {
            for i in 0..1024 {
                vec_a[i] = vec_b[i]
            }
        });
    }
}
