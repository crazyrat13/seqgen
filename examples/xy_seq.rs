//! x-y sequence example.

use seqgen::prelude::*;

fn main() {
    let initial_elements = vec![String::from("x"), String::from("y")];
    let mut seq = Sequence::new()
        .initial_elements(initial_elements)
        .transition_function(|alive_elements, current_element_index| {
            let mut next = String::from(
                alive_elements
                    .nth_element(current_element_index - 2)
                    .unwrap(),
            );

            next.push_str(
                alive_elements
                    .nth_element(current_element_index - 1)
                    .unwrap(),
            );

            next
        });

    seq.range(0, 12)
        .unwrap()
        .for_each(|element| println!("{element}"));
}
