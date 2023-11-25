# SeqGen

## Sequence generation library

SeqGen is a library written in Rust that lets you generate sequences in a performant and easy way.

SeqGen is lazy meaning the elements won't be generated until needed or explicitly requested.

The sequences generated by SeqGen implement the <code>Iterator</code> trait so manipulation is done using familiar rust iterators syntax.

### New is this release:

- Making low level types and states public

### Install:

From the commend line in your project directory run the following

```console
$ cargo add seqgen
```

Or add this under <code>[[dependencies]]</code> in <code>Cargo.toml</code> file

```toml
seqgen = "0.3.2"
```

### Examples:

#### Fibonacci sequence

```rust
use seqgen::prelude::*;

fn main() {
    let fib_seq = Sequence::new()
        .initial_elements(vec![0, 1_u128])
        .transition_function(|alive_elements, current_index| {
            alive_elements.nth_element(current_index - 1).unwrap()
                + alive_elements.nth_element(current_index - 2).unwrap()
        })
        .pre_generate(185); // more than 185 would cause u128 overflow.

    fib_seq
        .alive_elements()
        .for_each(|element| println!("{element}"));
}
```

#### An x-y sequence

```rust
use seqgen::prelude::*;

fn main() {
    let mut seq = Sequence::new()
        .initial_elements(vec!["x".to_string(), "y".to_string()])
        .transition_function(|alive_elements, current_element_index| {
            let mut string = String::from(
                alive_elements
                    .nth_element(current_element_index - 2)
                    .unwrap(),
            );

            string.push_str(
                alive_elements
                    .nth_element(current_element_index - 1)
                    .unwrap(),
            );

            string
        });

    seq.range(0, 10)
        .unwrap()
        .for_each(|element| println!("{element}"));
}
```

#### Range of sequence

```rust
use seqgen::prelude::*;

fn main() {
    let mut seq = Sequence::linear_seq();
    let range_res = seq.range_mut(10000000, 10000000000);

    if let Ok(range) = range_res {
        range.for_each(|element| println!("{element}"))
    }
}
```

See more examples on the [examples folder](https://github.com/crazyrat13/seqgen/tree/main/examples).

Have suggestions? [contribute to the project or open an issue on GitHub](https://github.com/crazyrat13/seqgen).
