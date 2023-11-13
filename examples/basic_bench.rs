//! A basic benchmark example.

use std::time::SystemTime;

use seqgen::prelude::*;

fn main() {
    let mut seq = Sequence::linear_seq();

    let n = 1_000_000_000;
    let now = SystemTime::now();
    let element = seq.nth_element(n);
    let later = SystemTime::now();
    let delta_secs = later.duration_since(now).unwrap().as_secs_f64();

    println!("Generated {n} elements in: {delta_secs}s");
    println!("Last alive element: {element}");
}
