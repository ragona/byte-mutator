# byte-mutator
`byte-mutator` is a crate for defining a set of rules by which to mutate byte arrays. It's intented to be used as part
of a fuzzing workflow to configure how you want your input mutated. For example, you might want to do one pass where 
you don't mess with the header of your message, and you only mutate the body -- or you could mutate them differently. 

## Examples

### BitFlipper
This example is configured to flip every bit in the bytes one at a time.

```rust
let mut bytes = b"foo".to_vec();
let mut mutator = ByteMutator::new().with_stages(vec![Stage {
    count: 0,
    max: Some(24),
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
```

### Load from config
This is an example of a mutator configured to flip bits forever. 
```toml
[[stages]]
    # Iteration count at which to start the loop (useful for starting over from a future state)
    count = 0

    # A list of mutations to perform on this stage
    [[stages.mutations]]
        # Must be a variant of the MutatorTypes enum
        mutation = {"BitFlipper" = {width = 1 }}
```

## Release History

* 0.2.0
    * Removed data structure from ByteMutator. Mutation now requires a `&mut [u8]` reference.
    * Removed `Iterations`, just added a simple `Option<usize>` for `max`. 
* 0.1.0
    * Initial release

## Meta

Ryan Ragona – [@ryanragona](https://twitter.com/ryanragona) – [https://github.com/ragona](https://github.com/ragona/)

Distributed under the MTT license. See ``LICENSE`` for more information.

## Contributing
Always happy to see PRs or Issues. 

To contribute: 
1. Fork it (<https://github.com/ragona/yourproject/byte-mutator>)
2. Create your feature branch (`git checkout -b feature/fooBar`)
3. Commit your changes (`git commit -am 'Add some fooBar'`)
4. Push to the branch (`git push origin feature/fooBar`)
5. Create a new Pull Requesti