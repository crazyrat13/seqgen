use seqgen::prelude::*;

fn main() {
    let mut seq = Sequence::new()
        .initial_elements(vec!["x".to_string(), "y".to_string()])
        .transition_function(|alive_elements, current_element_index| {
            let mut string = String::from(
                alive_elements
                    .nth_element(current_element_index - 2)
                    .unwrap(),
            );

            string.push_str(
                alive_elements
                    .nth_element(current_element_index - 1)
                    .unwrap(),
            );

            string
        });

    seq.range(0, 12)
        .unwrap()
        .for_each(|element| println!("{element}"));
}
