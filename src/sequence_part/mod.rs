//! This module defines the SequencePart type
//! that represents a part of a Sequence type

pub mod states;

use states::*;

use super::sequence::{states::*, Sequence};

/// This type represents a part of a sequence.
/// it could be the alive elements or a range of the sequence
/// This type is used only to read from not to write to the sequence
pub struct SequencePart<'a, T, P> {
    parent_sequence: &'a Sequence<T, WithInitialElements, WithTransitionFunction<T>>,
    state: P,
    iter_index: usize,
}

impl<'a, T> SequencePart<'a, T, AliveElements> {
    /// Create a new instance that represents the alive elements
    pub(super) fn new(
        parent_sequence: &'a Sequence<T, WithInitialElements, WithTransitionFunction<T>>,
    ) -> SequencePart<'a, T, AliveElements> {
        SequencePart {
            parent_sequence,
            state: AliveElements,
            iter_index: 0,
        }
    }

    /// Returns the length of the alive elements of the sequence
    pub fn len(&self) -> usize {
        self.parent_sequence.alive_elements_len()
    }

    /// Checks if the alive elements are empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the nth elements of the alive elements
    pub fn nth_element(&self, index: usize) -> Option<&T> {
        self.parent_sequence.nth_element_without_generation(index)
    }

    /// Returns the first elements of the alive elements
    pub fn first_element(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        self.nth_element(0)
    }

    /// Returns the last elements of the alive elements
    pub fn last_element(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        self.nth_element(self.len() - 1)
    }
}

impl<'a, T> SequencePart<'a, T, Range> {
    /// Create a new instance that represents a range of a sequence
    pub(super) fn new_range(
        parent_sequence: &'a Sequence<T, WithInitialElements, WithTransitionFunction<T>>,
        start: usize,
        end: usize,
    ) -> SequencePart<'a, T, Range> {
        SequencePart {
            parent_sequence,
            state: Range::new(start, end),
            iter_index: start,
        }
    }

    /// Returns the nth elements of the range
    pub fn nth_element(&self, index: usize) -> Option<&T> {
        if !self.nth_element_is_in_range(index) {
            return None;
        }

        self.parent_sequence.nth_element_without_generation(index)
    }

    /// Checks if the nth element is in range
    pub fn nth_element_is_in_range(&self, index: usize) -> bool {
        index >= self.state.start() && index < self.state.end()
    }
}

impl<'a, T: Clone> Iterator for SequencePart<'a, T, AliveElements> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let iter_index = self.iter_index;

        if iter_index == self.len() {
            self.iter_index = 0;
            return None;
        }

        self.iter_index += 1;
        self.nth_element(iter_index).cloned()
    }
}

impl<'a, T: Clone> Iterator for SequencePart<'a, T, Range> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let iter_index = self.iter_index;

        if iter_index == self.state.end() {
            self.iter_index = 0;
            return None;
        }

        self.iter_index += 1;
        self.nth_element(iter_index).cloned()
    }
}
