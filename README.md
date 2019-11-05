# byte-mutator
`byte-mutator` is a crate for defining a set of rules by which to mutate byte arrays. It's intented to be used as part
of a fuzzing workflow to configure how you want your input mutated. For example, you might want to do one pass where 
you don't mess with the header of your message, and you only mutate the body -- or you could mutate them differently. 

## Example 
This is an example of a mutator configured to flip bits forever. 
```toml
[[stages]]
    # Iteration count at which to start the loop (useful for starting over from a future state)
    count = 0
    # Optional range to limit the number of times that this stage runs
    iterations = "Unlimited"

    # A list of mutations to perform on this stage
    [[stages.mutations]]
        # Must be a variant of the MutatorTypes enum
        mutation = {"BitFlipper" = {width = 1 }}
```

```rust
let mut bytes = ByteMutator::new_from_config(b"foo", FuzzConfig::from_file("config.toml"));

for _ in 0..20 {
    // this advances the state by one step
    bytes.next();
    // each time this will be one bit different from the original
    dbg!(bytes.read());
}
```

## Release History

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