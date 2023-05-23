use crate::sequence_generator::SequenceGenerator;

use std::marker::PhantomData;

pub mod sequence_states {
    pub struct Defined;
    pub struct Undefined;
}

use sequence_states::Defined;
use sequence_states::Undefined;

pub struct Sequence<'a, T, S> {
    generator: Option<&'a SequenceGenerator<T>>,
    rest_of_elements: Vec<T>,
    iter_index: usize,
    state: PhantomData<S>,
}

impl<'a, T, S> Sequence<'a, T, S> {
    pub(crate) fn new(generator: &'a SequenceGenerator<T>, rest_of_elements: Vec<T>) -> Self {
        Self {
            generator: Some(generator),
            rest_of_elements,
            iter_index: 0,
            state: PhantomData,
        }
    }

    pub(crate) fn rest_of_elements(&self) -> &Vec<T> {
        &self.rest_of_elements
    }

    pub(crate) fn rest_of_elements_len(&self) -> usize {
        self.rest_of_elements.len()
    }
}

impl<'a, T, S> Sequence<'a, T, S> {
    /// Returns the length of the sequence
    pub fn len(&self) -> usize {
        let mut len = self.rest_of_elements_len();

        if let Some(generator) = self.generator {
            len += generator.initial_elements_len();
        }

        len
    }

    /// Returns the nth element of the sequence
    pub fn nth_element(&self, index: usize) -> Option<&T> {
        let index = match self.generator {
            None => index,
            Some(generator) => {
                let initial_elements_len = generator.initial_elements_len();

                if index < initial_elements_len {
                    return generator.nth_initial_element(index);
                }

                index - initial_elements_len
            }
        };

        if index < self.rest_of_elements.len() {
            return Some(&self.rest_of_elements[index]);
        }

        None
    }
}

impl<'a, T> Sequence<'a, T, Defined> {
    pub(crate) fn new_from_undefined(
        undefined_seq: Sequence<'a, T, Undefined>,
        generator: &'a SequenceGenerator<T>,
    ) -> Self {
        Self {
            generator: Some(generator),
            rest_of_elements: undefined_seq.rest_of_elements,
            iter_index: undefined_seq.iter_index,
            state: PhantomData,
        }
    }

    /// Extends the sequence with a number of elements given as argument
    pub fn extend(&mut self, number_of_elements: usize) {
        self.rest_of_elements.extend(
            self.generator
                .unwrap()
                .extend_sequence(&self, number_of_elements)
                .into_iter(),
        );
    }

    /// Reset the sequence
    pub fn reset(&mut self) {
        self.rest_of_elements = Vec::new();
        self.iter_index = 0;
    }

    /// Reset the sequence and regenerate it with a number of elements given as argument
    pub fn regenerate(&mut self, number_of_elements: usize) {
        self.reset();
        self.extend(number_of_elements);
    }
}

impl<'a, T> Sequence<'a, T, Undefined> {
    pub(crate) fn new_undefined(rest_of_elements: Vec<T>) -> Self {
        Self {
            generator: None,
            rest_of_elements,
            iter_index: 0,
            state: PhantomData,
        }
    }

    pub fn define(self, generator: &'a SequenceGenerator<T>) -> Sequence<'a, T, Defined> {
        Sequence::new_from_undefined(self, generator)
    }
}

impl<'a, A> FromIterator<A> for Sequence<'a, A, Undefined> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut rest_of_elements = Vec::new();

        for element in iter {
            rest_of_elements.push(element);
        }

        Sequence::new_undefined(rest_of_elements)
    }
}

impl<T: Clone, S> Iterator for Sequence<'_, T, S> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let iter_index = self.iter_index;

        if iter_index >= self.len() {
            self.iter_index = 0;
            return None;
        }

        self.iter_index += 1;

        match self.nth_element(iter_index) {
            None => None,
            Some(element) => Some(element.clone()),
        }
    }
}
