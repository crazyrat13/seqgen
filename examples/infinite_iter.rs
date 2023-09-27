use seqgen::prelude::*;

fn main() {
    Sequence::new()
        .transition_function(|_, current_index| current_index)
        .for_each(|element| println!("{element}"));
}
