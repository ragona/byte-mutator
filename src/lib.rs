use crate::mutators::Mutate;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mutators::bitflipper::BitFlipper;
    use crate::mutators::{Mutate, Range};

    #[test]
    fn single_mutation() {
        let mut buffer = b"foo".to_vec();
        let mut mutator = BitFlipper::new();

        mutator.mutate(&mut buffer);
        mutator.mutate(&mut buffer);
        mutator.mutate(&mut buffer);

        dbg!(buffer);
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
}
