//! This module defines the Sequence type
//! and it is the core of this library

pub mod states;
pub mod types;

use self::{states::*, types::TransitionFunction};

use crate::sequence_part::{
    error::RangeError,
    types::{
        AliveElementsPart, RangePartImmut, RangePartImmutResult, RangePartMut, RangePartMutResult,
    },
};

/// A type that represents a sequence.
/// the Sequence type uses Vec to store its elements,
/// so the max number of elements
/// it can hold is std::usize::MAX.
pub struct Sequence<T, I, F> {
    initial_elements: I,
    trans_func: F,
    alive_elements: Vec<T>,
    iter_index: usize,
}

/// Shared behavior between sequences that requires initial elements
/// and sequences that do not require initial elements.
pub trait SharedSequenceBehavior {
    /// Pre generate the specified number elements on the sequence.
    fn pre_generate(self, number_of_elements: usize) -> Self;

    /// Removes all alive (generated) elements.
    fn clear(&mut self);
}

impl<T> Default for Sequence<T, WithoutInitialElements, WithoutTransitionFunction> {
    /// Creates a default instance of Sequence (undefined sequence).
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
    /// Creates a new instance of Sequence which is an undefined Sequence.
    pub fn new() -> Self {
        Sequence::default()
    }

    /// Adds initial elements to the sequence.
    pub fn initial_elements(
        self,
        initial_elements: Vec<T>,
    ) -> Sequence<T, WithInitialElements, WithoutTransitionFunction> {
        Sequence {
            initial_elements: WithInitialElements::new(initial_elements.len()),
            trans_func: self.trans_func,
            alive_elements: initial_elements,
            iter_index: self.iter_index,
        }
    }
}

impl<T, F> Sequence<T, WithInitialElements, F> {
    /// Returns the length of the initial elements.
    pub fn initial_elements_len(&self) -> usize {
        self.initial_elements.len()
    }
}

impl<T, I> Sequence<T, I, WithoutTransitionFunction> {
    /// Adds transition function to the sequence.
    pub fn transition_function(
        self,
        trans_func: TransitionFunction<T, I>,
    ) -> Sequence<T, I, WithTransitionFunction<T, I>> {
        Sequence {
            initial_elements: self.initial_elements,
            trans_func: WithTransitionFunction::new(trans_func),
            alive_elements: self.alive_elements,
            iter_index: self.iter_index,
        }
    }
}

impl<T, I> Sequence<T, I, WithTransitionFunction<T, I>> {
    /// Generates the nth element and all the preceding elements and stores them in the sequence.
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

    /// Returns a reference to the nth element if it is alive in a Some variant
    /// This method does not generate the nth elements if it is dead it just returns None.
    pub(crate) fn nth_element_without_generation(&self, index: usize) -> Option<&T> {
        if !self.nth_element_is_alive(index) {
            return None;
        }

        Some(&self.alive_elements[index])
    }

    /// Generates the specified number of elements.
    pub fn generate(&mut self, number_of_elements: usize) {
        self.generate_nth_element(self.alive_elements_len() + number_of_elements - 1);
    }

    /// Returns the length of the alive elements.
    pub fn alive_elements_len(&self) -> usize {
        self.alive_elements.len()
    }

    /// Checks if an element is alive.
    pub fn nth_element_is_alive(&self, index: usize) -> bool {
        index < self.alive_elements_len()
    }

    /// Returns a reference to the nth element if it is alive.
    /// This method generate the nth element if it is dead before returning its reference.
    pub fn nth_element(&mut self, index: usize) -> &T {
        self.generate_nth_element(index);
        &self.alive_elements[index]
    }

    /// Returns a sequence part that represents the alive elements.
    pub fn alive_elements(&self) -> AliveElementsPart<'_, T, I> {
        AliveElementsPart::new(self)
    }

    /// Returns a sequence part that represents an immutable range of the sequence.
    pub fn range(&self, start: usize, end: usize) -> RangePartImmutResult<'_, T, I> {
        if start > end {
            return Err(RangeError::InvalidRange { start, end });
        }

        if !self.nth_element_is_alive(end - 1) {
            return Err(RangeError::DeadRange);
        }

        Ok(RangePartImmut::new_range(self, start, end))
    }

    /// Returns a sequence part that represents a mutable range of the sequence.
    pub fn range_mut(&mut self, start: usize, end: usize) -> RangePartMutResult<'_, T, I> {
        if start > end {
            return Err(RangeError::InvalidRange { start, end });
        }

        Ok(RangePartMut::new_range_mut(self, start, end))
    }
}

impl
    Sequence<usize, WithoutInitialElements, WithTransitionFunction<usize, WithoutInitialElements>>
{
    /// Returns a linear sequence.
    pub fn linear_seq() -> Self {
        Sequence::new().transition_function(|_, i| i)
    }
}

impl<T> SharedSequenceBehavior
    for Sequence<T, WithInitialElements, WithTransitionFunction<T, WithInitialElements>>
{
    fn pre_generate(mut self, number_of_elements: usize) -> Self {
        if number_of_elements > 0 {
            let initial_elements_len = self.initial_elements_len();
            let last_generated_element = number_of_elements - 1 + initial_elements_len;
            self.generate_nth_element(last_generated_element);
        }

        self
    }

    fn clear(&mut self) {
        let mut initial_elements = Vec::new();

        for index in 0..self.initial_elements_len() {
            initial_elements.push(self.alive_elements.remove(index));
        }

        self.alive_elements = initial_elements;
    }
}

impl<T> SharedSequenceBehavior
    for Sequence<T, WithoutInitialElements, WithTransitionFunction<T, WithoutInitialElements>>
{
    fn pre_generate(mut self, number_of_elements: usize) -> Self {
        if number_of_elements != 0 {
            self.generate_nth_element(number_of_elements - 1);
        }

        self
    }

    fn clear(&mut self) {
        self.alive_elements = Vec::new();
    }
}

impl<T: Clone, I> Iterator for Sequence<T, I, WithTransitionFunction<T, I>> {
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
