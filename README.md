# seqgen

## Sequence generation library

### Install:

From the commend line in your project directory run the following

```console
$ cargo add seqgen
```

Or add this under <code>[[dependencies]]</code> in <code>Cargo.toml</code>

```toml
seqgen = "0.1.4"
```

### Examples:

#### Fibonacci

```rust
use seqgen::SequenceGenerator;

fn main() {
    let initial_elements = vec![0, 1];
    let trans_func = |current_elements: Vec<&u128>| -> u128 {
        current_elements[current_elements.len() - 1] + current_elements[current_elements.len() - 2]
    };

    let generator = SequenceGenerator::new(initial_elements, trans_func);
    let seq = generator.generate(187);

    seq.into_iter().for_each(|element| println!("{element}"));
}

```

#### An x-y sequence

```rust
use seqgen::SequenceGenerator;

fn main() {
    let initial_elements = vec![String::from("x"), String::from("y")];
    let trans_func = |current_elements: Vec<&String>| -> String {
        let mut string = String::from(current_elements[current_elements.len() - 2]);
        string.push_str(current_elements[current_elements.len() - 1]);
        string
    };

    let generator = SequenceGenerator::new(initial_elements, trans_func);
    let seq = generator.generate(10);

    seq.into_iter().for_each(|element| println!("{element}"));
}

```

#### Sequence from an iterator

```rust
use seqgen::{Sequence, SequenceGenerator};

fn main() {
    let undefined_seq = Sequence::from_iter(vec![1, 2, 3]);
    let initial_elements = vec![0];
    let trans_func = |current: Vec<&u128>| current[current.len() - 1] + 1;
    let generator = SequenceGenerator::new(initial_elements, trans_func);
    let mut defined_seq = undefined_seq.define(&generator);

    defined_seq.extend(7);
    defined_seq.into_iter().for_each(|element| println!("{element}"));
}

```

If you find this project useful [give it a star on GitHub](https://github.com/crazyrat13/seqgen).

If you have some idea in mind to make this
project useful [contact us via bjohn4412@gmail.com](mailto:bjohn4412@gmail.com).
