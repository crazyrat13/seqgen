//! This module defines the SequencePart type
//! that represents a part of a Sequence type.

//! A sequence part could alive elements part
//! that represents the alive elements of the sequence
//! or range part (custom range) that represents a range of a sequence.

pub mod states;
pub mod types;

use states::*;
use types::{AliveElementsPart, ParentSequenceRef, ParentSequenceRefMut, RangePart};

/// This type represents a part of a sequence.
/// it could be the alive elements or a range of the sequence
/// This type is used only to read from not to write to the sequence
pub struct SequencePart<P, S> {
    parent_sequence: S,
    state: P,
    iter_index: usize,
}

/// Shared behavior between Range and AliveElements
pub trait SharedSequencePartBehavior<T> {
    /// Returns the length of the sequence part.
    fn len(&self) -> usize;

    /// Checks if the sequence part is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the first element of the sequence part.
    fn first_element(&mut self) -> Option<&T>;

    /// Returns the last element of the sequence part.
    fn last_element(&mut self) -> Option<&T>;
}

impl<'a, T, I> AliveElementsPart<'a, T, I> {
    /// Create a new instance that represents the alive elements
    pub(super) fn new(parent_sequence: ParentSequenceRef<'a, T, I>) -> AliveElementsPart<'a, T, I> {
        SequencePart {
            parent_sequence,
            state: AliveElements,
            iter_index: 0,
        }
    }

    /// Returns the nth elements of the alive elements
    pub fn nth_element(&self, index: usize) -> Option<&T> {
        self.parent_sequence.nth_element_without_generation(index)
    }
}

impl<'a, T, I> RangePart<'a, T, I> {
    /// Create a new instance that represents a range of a sequence
    pub(super) fn new_range(
        parent_sequence: ParentSequenceRefMut<'a, T, I>,
        start: usize,
        end: usize,
    ) -> RangePart<'a, T, I> {
        SequencePart {
            parent_sequence,
            state: Range::new(start, end),
            iter_index: start,
        }
    }

    /// Check if the element is in the range
    pub fn nth_element_is_in_range(&self, index: usize) -> bool {
        index >= self.state.start() && index < self.state.end()
    }

    /// Returns the nth elements of the range part
    pub fn nth_element(&mut self, index: usize) -> Option<&T> {
        if !self.nth_element_is_in_range(index) {
            return None;
        }

        Some(self.parent_sequence.nth_element(index))
    }
}

impl<'a, T, I> SharedSequencePartBehavior<T> for AliveElementsPart<'a, T, I> {
    fn len(&self) -> usize {
        self.parent_sequence.alive_elements_len()
    }

    fn first_element(&mut self) -> Option<&T> {
        self.nth_element(0)
    }

    fn last_element(&mut self) -> Option<&T> {
        self.nth_element(self.len() - 1)
    }
}

impl<'a, T, I> SharedSequencePartBehavior<T> for RangePart<'a, T, I> {
    fn len(&self) -> usize {
        self.state.end() - self.state.start()
    }

    fn first_element(&mut self) -> Option<&T> {
        self.nth_element(self.state.start())
    }

    fn last_element(&mut self) -> Option<&T> {
        self.nth_element(self.state.end())
    }
}

impl<'a, T: Clone, I> Iterator for AliveElementsPart<'a, T, I> {
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

impl<'a, T: Clone, I> Iterator for RangePart<'a, T, I> {
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
