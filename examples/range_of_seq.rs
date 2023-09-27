use seqgen::prelude::*;

fn main() {
    let mut seq = Sequence::new().transition_function(|_, current_index| current_index);
    let range_res = seq.range(0, 10);

    if let Ok(range) = range_res {
        range.for_each(|element| println!("{element}"));
    }
}
