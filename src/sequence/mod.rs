//! This module defines the Sequence type
//! and it is the core of this library

pub mod states;
pub mod types;

use self::{states::*, types::TransitionFunction};
use super::error::{RangeError, RangeErrorKind};
use super::sequence_part::{
    states::{AliveElements, Range},
    SequencePart,
};

/// A type that represents a sequence.
/// the Sequence type uses Vec to store its elements, so the max number of elements
/// it can hold is std::usize::MAX, that's enough for most of us :)
pub struct Sequence<T, I, F> {
    initial_elements: I,
    trans_func: F,
    alive_elements: Vec<T>,
    iter_index: usize,
}

impl<T> Default for Sequence<T, WithoutInitialElements, WithoutTransitionFunction> {
    /// Creates a default instance of Sequence
    fn default() -> Self {
        Self {
            initial_elements: WithoutInitialElements,
            trans_func: WithoutTransitionFunction,
            alive_elements: Vec::new(),
            iter_index: 0,
        }
    }
}

impl<T> Sequence<T, WithoutInitialElements, WithoutTransitionFunction> {
    /// Creates a new instance of Sequence
    pub fn new() -> Self {
        Sequence::default()
    }
}

impl<T, F> Sequence<T, WithoutInitialElements, F> {
    /// Adds initial elements to Sequence
    pub fn initial_elements(self, initial_elements: Vec<T>) -> Sequence<T, WithInitialElements, F> {
        let initial_elements_len = initial_elements.len();
        let alive_elements = initial_elements
            .into_iter()
            .chain(self.alive_elements.into_iter())
            .collect::<Vec<_>>();

        Sequence {
            initial_elements: WithInitialElements::new(initial_elements_len),
            trans_func: self.trans_func,
            alive_elements,
            iter_index: self.iter_index,
        }
    }
}

impl<T, F> Sequence<T, WithInitialElements, F> {
    /// Returns the length of the initial elements
    pub fn initial_elements_len(&self) -> usize {
        self.initial_elements.len()
    }
}

impl<T, I> Sequence<T, I, WithoutTransitionFunction> {
    /// Adds transition function to Sequence
    pub fn transition_function(
        self,
        trans_func: TransitionFunction<T>,
    ) -> Sequence<T, I, WithTransitionFunction<T>> {
        Sequence {
            initial_elements: self.initial_elements,
            trans_func: WithTransitionFunction::new(trans_func),
            alive_elements: self.alive_elements,
            iter_index: self.iter_index,
        }
    }
}

impl<T> Sequence<T, WithInitialElements, WithTransitionFunction<T>> {
    /// Pre generate elements on Sequence
    pub fn pre_generated(mut self, number_of_elements: usize) -> Self {
        self.generate_nth_element(number_of_elements - self.initial_elements_len());
        self
    }

    /// Returns the length of the alive elements
    pub fn alive_elements_len(&self) -> usize {
        self.alive_elements.len()
    }

    /// Checks if an elements is alive
    pub fn nth_element_is_alive(&self, index: usize) -> bool {
        index < self.alive_elements_len()
    }

    /// Generates the nth element (and all the preceding elements)
    fn generate_nth_element(&mut self, nth_element: usize) {
        if !self.nth_element_is_alive(nth_element) {
            let alive_elements_len = self.alive_elements_len();

            for current_element_index in alive_elements_len..=nth_element {
                let alive_elements_part = self.alive_elements();
                let new_element = self
                    .trans_func
                    .run(alive_elements_part, current_element_index);
                self.alive_elements.push(new_element);
            }
        }
    }

    /// Returns a reference to the nth element if it is alive.
    /// This method generate the nth elements if it is dead before returning it
    pub fn nth_element(&mut self, index: usize) -> &T {
        if !self.nth_element_is_alive(index) {
            self.generate_nth_element(index);
        }

        &self.alive_elements[index]
    }

    /// Returns a reference the nth element if it is alive
    /// This method does not generate the nth elements if it is dead it just returns None
    pub(super) fn nth_element_without_generation(&self, index: usize) -> Option<&T> {
        if !self.nth_element_is_alive(index) {
            return None;
        }

        Some(&self.alive_elements[index])
    }

    /// Returns a sequence part that represents the alive elements
    pub fn alive_elements(&self) -> SequencePart<'_, T, AliveElements> {
        SequencePart::new(self)
    }

    /// Returns a sequence part that represents a range of sequence
    pub fn range(
        &mut self,
        start: usize,
        end: usize,
    ) -> Result<SequencePart<'_, T, Range>, RangeError> {
        if start > end {
            return Err(RangeError::new(RangeErrorKind::InvalidRange));
        }

        if !self.nth_element_is_alive(end) && start != end {
            self.generate_nth_element(end)
        }

        Ok(SequencePart::new_range(self, start, end))
    }
}

impl<T: Clone> Iterator for Sequence<T, WithInitialElements, WithTransitionFunction<T>> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let iter_index = self.iter_index;

        if iter_index == std::usize::MAX {
            self.iter_index = 0;
            return None;
        }

        self.iter_index += 1;

        Some(self.nth_element(iter_index).clone())
    }
}
