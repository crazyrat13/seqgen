// use seqgen::prelude::seq_without_initial_elements;
use seqgen::prelude::*;

fn main() {
    Sequence::new()
        .transition_function(|_, i| i)
        .for_each(|element| println!("{element}"))
}
