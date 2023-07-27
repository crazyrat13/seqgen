use seqgen::prelude::*;

fn main() {
    let seq = Sequence::new()
        .initial_elements(vec![])
        .transition_function(|alive_elements, current_element_index| {
            alive_elements
                .last_element()
                .map_or(current_element_index, |element| element + 1)
        });

    seq.for_each(|element| println!("{element}"));
}
