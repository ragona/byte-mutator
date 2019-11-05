extern crate byte_mutator;

use byte_mutator::*;

#[test]
fn mutator_from_config() {
    let mut bytes = ByteMutator::new_from_config(b"foo", FuzzConfig::default());

    for _ in 0..20 {
        bytes.next();
    }

    assert!(bytes.remaining_stages() >= 1);
}

#[test]
fn mutator() {
    let mut bytes = ByteMutator::new(b"foo").with_stages(vec![Stage {
        count: 0,
        iterations: Iterations::Bits,
        mutations: vec![Mutation {
            range: None,
            mutation: MutationType::BitFlipper { width: 1 },
        }],
    }]);

    // Bytes in their original state
    assert_eq!(bytes.read(), b"foo");

    // Advance the mutation
    bytes.next();

    // We've flipped the first bit (little endian)
    // 0b1100110 -> 0b1100111, 103 -> 102, f -> g
    assert_eq!(bytes.read(), b"goo");
}
