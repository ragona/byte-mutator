extern crate byte_mutator;

use byte_mutator::*;

#[test]
fn mutator_from_config() {
    let mut bytes = ByteMutator::new_from_config(FuzzConfig::default());

    for _ in 0..20 {
        bytes.next();
    }

    assert!(bytes.remaining_stages() >= 1);
}

#[test]
fn mutator() {
    let mut bytes = b"foo".to_vec();
    let mut mutator = ByteMutator::new().with_stages(vec![Stage {
        count: 0,
        max: Some(10),
        mutations: vec![Mutation {
            range: None,
            mutation: MutationType::BitFlipper { width: 1 },
        }],
    }]);

    // Bytes in their original state
    assert_eq!(&bytes, b"foo");

    // Perform a single mutation
    mutator.mutate(&mut bytes);

    // We've flipped the first bit (little endian)
    // 0b1100110 -> 0b1100111, 103 -> 102, f -> g
    assert_eq!(&bytes, b"goo");
}
