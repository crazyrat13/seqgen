use seqgen::prelude::seq_without_initial_elements;

fn main() {
    seq_without_initial_elements(|_, current_index| current_index)
        .for_each(|element| println!("{element}"));
}
