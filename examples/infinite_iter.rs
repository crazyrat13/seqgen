//! Infinite iterator over a sequence example.

use seqgen::prelude::Sequence;

fn main() {
    Sequence::linear_seq().for_each(|e| println!("{e}"))
}
