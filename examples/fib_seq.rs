use seqgen::SequenceGenerator;

fn main() {
    let initial_elements = vec![0, 1];
    let trans_func = |current_elements: Vec<&u128>| -> u128 {
        current_elements[current_elements.len() - 1] + current_elements[current_elements.len() - 2]
    };

    let generator = SequenceGenerator::new(initial_elements, trans_func);
    let seq = generator.generate(187);

    seq.into_iter().for_each(|element| println!("{element}"));
}
