use std::time::SystemTime;

use seqgen::prelude::*;

fn main() {
    let mut seq = Sequence::new()
        .initial_elements(vec![])
        .transition_function(|alive_elements, current_element_index| {
            match alive_elements.last_element() {
                None => current_element_index,
                Some(element) => {
                    // println!("{element}");
                    element + 1_usize
                }
            }
        });

    let n = 100_000_000;
    let now = SystemTime::now();
    let element = seq.nth_element(n);
    let later = SystemTime::now();
    let delta_secs = later.duration_since(now).unwrap().as_secs_f64();

    println!("Generated {n} elements in: {delta_secs}s");
    println!("Last alive element: {element}")
}
