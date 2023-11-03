use seqgen::prelude::*;

fn main() {
    let mut seq = Sequence::new().transition_function(|_, current_index| current_index);
    let range_res = seq.range(10000000000, 10000000);

    match range_res {
        Ok(range) => range.for_each(|element| println!("{element}")),
        Err(error) => match error.kind {
            RangeErrorKind::InvalidRange { .. } => eprintln!("{error:?}")
        },
    }
}
