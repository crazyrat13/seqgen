# seqgen

## Sequence generation library

### Install:
From the commend line in your project directory run the following 
```console
$ cargo add seqgen
```
Or add this under <code>[[dependencies]]</code> in <code>Cargo.toml</code>

```toml
seqgen = "0.1.0"
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

    seq.for_each(|element| println!("{element}"));
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

    seq.for_each(|element| println!("{element}"));
}

```
