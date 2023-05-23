use seqgen::SequenceGenerator;

fn main() {
    let initial_elements = vec![String::from("x"), String::from("y")];
    let trans_func = |current_elements: Vec<&String>| -> String {
        let mut string = String::from(current_elements[current_elements.len() - 2]);
        string.push_str(current_elements[current_elements.len() - 1]);
        string
    };

    let generator = SequenceGenerator::new(initial_elements, trans_func);
    let seq = generator.generate(10);

    seq.for_each(|element| println!("{element}"));
}
