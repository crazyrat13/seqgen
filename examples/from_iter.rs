use seqgen::{Sequence, SequenceGenerator};

fn main() {
    let undefined_seq = Sequence::from_iter(vec![1, 2, 3]);
    let initial_elements = vec![0];
    let trans_func = |current: Vec<&u128>| current[current.len() - 1] + 1;
    let generator = SequenceGenerator::new(initial_elements, trans_func);
    let mut defined_seq = undefined_seq.define(&generator);

    defined_seq.extend(7);
    defined_seq.for_each(|element| println!("{element}"));
}
