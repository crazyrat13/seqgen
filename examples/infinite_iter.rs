use seqgen::prelude::Sequence;

fn main() {
    Sequence::new()
        .transition_function(|_, i| i)
        .for_each(|e| println!("{e}"))
}
