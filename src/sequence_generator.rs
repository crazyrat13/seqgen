use crate::{sequence::sequence_states::Defined, Sequence};

pub(crate) type TransFunc<T> = fn(Vec<&T>) -> T;

pub struct SequenceGenerator<T> {
    initial_elements: Vec<T>,
    trans_func: TransFunc<T>,
}

impl<T> SequenceGenerator<T> {
    fn internal_generate(
        &self,
        intermediate_sequences: Option<Vec<&Vec<T>>>,
        number_of_elements: usize,
    ) -> Vec<T> {
        let mut generated_elements = Vec::new();

        for _ in 0..number_of_elements {
            let mut elements_ref = Vec::new();

            self.initial_elements
                .iter()
                .for_each(|element| elements_ref.push(element));

            if let Some(intermediate_sequences) = &intermediate_sequences {
                intermediate_sequences.iter().for_each(|&sequence| {
                    sequence
                        .iter()
                        .for_each(|element| elements_ref.push(element))
                });
            }

            generated_elements
                .iter()
                .for_each(|element| elements_ref.push(element));

            generated_elements.push((self.trans_func)(elements_ref));
        }

        generated_elements
    }
}

impl<T> SequenceGenerator<T> {
    pub(crate) fn extend_sequence<S>(
        &self,
        sequence: &Sequence<'_, T, S>,
        number_of_elements: usize,
    ) -> Vec<T> {
        let rest_of_elements = sequence.rest_of_elements();
        let intermediate_sequences = Some(vec![rest_of_elements]);
        self.internal_generate(intermediate_sequences, number_of_elements)
    }
}

impl<T> SequenceGenerator<T> {
    pub fn new(initial_elements: Vec<T>, trans_func: TransFunc<T>) -> Self {
        Self {
            initial_elements,
            trans_func,
        }
    }

    pub fn generate<'a>(&'a self, number_of_elements: usize) -> Sequence<'a, T, Defined> {
        let intermediate_sequences = None;
        let number_of_elements = number_of_elements - self.initial_elements_len();

        Sequence::new(
            &self,
            self.internal_generate(intermediate_sequences, number_of_elements),
        )
    }

    pub fn initial_elements(&self) -> &Vec<T> {
        &self.initial_elements
    }

    pub fn initial_elements_len(&self) -> usize {
        self.initial_elements.len()
    }

    pub fn nth_initial_element(&self, index: usize) -> Option<&T> {
        if index >= self.initial_elements_len() {
            return None;
        }

        Some(&self.initial_elements[index])
    }
}
