use seqgen::prelude::*;

fn main() {
    let seq = Sequence::new()
        .initial_elements(vec![])
        .transition_function(
            |alive_elements, current_element_index| match alive_elements.last() {
                None => current_element_index,
                Some(element) => element + 1_usize,
            },
        );

    seq.for_each(|element| println!("{element}"));
}
