# seqgen

## Sequence generation library

### New is this release:

- Better performance
- Better API

### Install:

From the commend line in your project directory run the following

```console
$ cargo add seqgen
```

Or add this under <code>[[dependencies]]</code> in <code>Cargo.toml</code>

```toml
seqgen = "0.2.0"
```

### Examples:

#### Fibonacci sequence

```rust
use seqgen::prelude::*;

fn main() {
    let seq = Sequence::new()
        .initial_elements(vec![0, 1_u128])
        .transition_function(|alive_elements, current_element_index| {
            alive_elements
                .nth_element(current_element_index - 1)
                .unwrap()
                + alive_elements
                    .nth_element(current_element_index - 2)
                    .unwrap()
        })
        .pre_generated(188);

    seq.alive_elements()
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
    let mut seq = Sequence::new()
        .initial_elements(vec![])
        .transition_function(|alive_elements, current_element_index| {
            alive_elements
                .last_element()
                .map_or(current_element_index, |element| element + 1)
        });

    let range_res: Result<SequencePart<'_, usize, Range>, RangeError> = seq.range(0, 10);

    if let Ok(range) = range_res {
        range.for_each(|element| println!("{element}"));
    }
}
```

If you find this project useful [give it a star on GitHub](https://github.com/crazyrat13/seqgen).
