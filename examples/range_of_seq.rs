//! A range of a sequence example.

use seqgen::prelude::*;

fn main() {
    let mut seq = Sequence::linear_seq();
    let range_res = seq.range_mut(10000000, 10000000000);

    if let Ok(range) = range_res {
        range.for_each(|element| println!("{element}"))
    }
}
