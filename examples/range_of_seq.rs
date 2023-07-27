use seqgen::prelude::*;

fn main() {
    let mut seq = Sequence::new()
        .initial_elements(vec![])
        .transition_function(|alive_elements, current_element_index| {
            alive_elements
                .last_element()
                .map_or(current_element_index, |element| element + 1)
        });

    let range_res: Result<SequencePart<'_, usize, Range>, RangeError> = seq.range(0, 10);

    if let Ok(range) = range_res {
        range.for_each(|element| println!("{element}"));
    }
}
