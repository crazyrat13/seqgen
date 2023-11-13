//! A range of a sequence example.

use seqgen::prelude::*;

fn main() {
    let mut seq = Sequence::linear_seq();
    let range_res = seq.range(10000000, 10000000000);

    match range_res {
        Ok(range) => range.for_each(|element| println!("{element}")),
        Err(error) => match error {
            RangeError::InvalidRange { .. } => eprintln!("Error: {error:?}"),
        },
    }
}
