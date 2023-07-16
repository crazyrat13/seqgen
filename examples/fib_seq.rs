use seqgen::prelude::*;

fn main() {
    let seq = Sequence::new()
        .initial_elements(vec![0, 1_u128])
        .transition_function(|alive_elements, current_element_index| {
            alive_elements
                .nth_element(current_element_index - 1)
                .unwrap()
                + alive_elements
                    .nth_element(current_element_index - 2)
                    .unwrap()
        })
        .pre_generated(188); // more than 188 will cause an overflow

    seq.alive_elements()
        .for_each(|element| println!("{element}"));
}
