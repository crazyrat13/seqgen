//! This module defines the sequence type

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
    state: PhantomData<S>,
}

impl<'a, T, S> Sequence<'a, T, S> {
    /// Creates a new sequence
    pub(crate) fn new(generator: &'a SequenceGenerator<T>, rest_of_elements: Vec<T>) -> Self {
        Self {
            generator: Some(generator),
            rest_of_elements,
            state: PhantomData,
        }
    }

    /// Returns a reference to the rest of elements of the sequence
    pub(crate) fn rest_of_elements(&self) -> &Vec<T> {
        &self.rest_of_elements
    }

    /// Returns the length of the rest of elements of the sequence
    pub(crate) fn rest_of_elements_len(&self) -> usize {
        self.rest_of_elements.len()
    }
}

impl<'a, T, S> Sequence<'a, T, S> {
    /// Check if the sequence is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

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
    /// Creates a new sequence from an undefined one
    pub(crate) fn new_from_undefined(
        undefined_seq: Sequence<'a, T, Undefined>,
        generator: &'a SequenceGenerator<T>,
    ) -> Self {
        Self {
            generator: Some(generator),
            rest_of_elements: undefined_seq.rest_of_elements,
            state: PhantomData,
        }
    }

    /// Extends the sequence with a number of elements given as argument
    pub fn extend(&mut self, number_of_elements: usize) {
        self.rest_of_elements.extend(
            self.generator
                .unwrap()
                .extend_sequence(self, number_of_elements)
                .into_iter(),
        );
    }

    /// Reset the sequence
    pub fn reset(&mut self) {
        self.rest_of_elements = Vec::new();
    }

    /// Reset the sequence and regenerate it with a number of elements given as argument
    pub fn regenerate(&mut self, number_of_elements: usize) {
        self.reset();
        self.extend(number_of_elements);
    }
}

impl<'a, T> Sequence<'a, T, Undefined> {
    /// Creates a new undefined sequence
    pub(crate) fn new_undefined(rest_of_elements: Vec<T>) -> Self {
        Self {
            generator: None,
            rest_of_elements,
            state: PhantomData,
        }
    }

    /// Define an undefined sequence (returns a defined one)
    pub fn define(self, generator: &'a SequenceGenerator<T>) -> Sequence<'a, T, Defined> {
        Sequence::new_from_undefined(self, generator)
    }
}

impl<T: Clone, S> Sequence<'_, T, S> {
    /// Returns a vector representing the sequence
    pub fn as_vec(&self) -> Vec<T> {
        let mut vec = Vec::new();

        match self.generator {
            None => (),
            Some(generator) => generator
                .initial_elements()
                .iter()
                .for_each(|element| vec.push(element.clone())),
        }

        self.rest_of_elements
            .iter()
            .for_each(|element| vec.push(element.clone()));

        vec
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

pub struct SequenceIter<'a, T, S> {
    sequence: Sequence<'a, T, S>,
    iter_index: usize,
}

impl<'a, T: Clone, S> IntoIterator for Sequence<'a, T, S> {
    type Item = T;

    type IntoIter = SequenceIter<'a, T, S>;

    fn into_iter(self) -> Self::IntoIter {
        SequenceIter::new(self)
    }
}

impl<'a, T, S> SequenceIter<'a, T, S> {
    /// Creates a new sequence iterator
    fn new(sequence: Sequence<'a, T, S>) -> Self {
        Self {
            sequence,
            iter_index: 0,
        }
    }

    /// Returns the underlying sequence
    pub fn as_seq(self) -> Sequence<'a, T, S> {
        self.sequence
    }
}

impl<T: Clone, S> Iterator for SequenceIter<'_, T, S> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let iter_index = self.iter_index;

        if iter_index >= self.sequence.len() {
            self.iter_index = 0;
            return None;
        }

        self.iter_index += 1;

        self.sequence.nth_element(iter_index).cloned()
    }
}
