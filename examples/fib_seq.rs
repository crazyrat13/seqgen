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
        .pre_generate(185); // more than 185 will cause a u_128 overflow

    seq.alive_elements()
        .for_each(|element| println!("{element}"));
}
