use seqgen::prelude::*;

fn main() {
    let mut seq = Sequence::new()
        .initial_elements(vec![])
        .transition_function(|alive_elements, current_element_index| {
            match alive_elements.last_element() {
                None => current_element_index,
                Some(element) => element + 1_usize,
            }
        });

    let range_res: Result<SequencePart<'_, usize, Range>, RangeError> = seq.range(0, 10);

    if let Ok(range) = range_res {
        range.for_each(|element| println!("{element}"));
    }
}
