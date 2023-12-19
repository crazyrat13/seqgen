//! This module defines the SequencePart type
//! that represents a part of a Sequence type.
//!
//! A sequence part could be alive elements part
//! that represents the alive elements of the sequence (Generated elements)
//! or range part (custom range) that represents a range of a sequence.
//!
//! The range parts are lazy and can mutate the sequence.
//! The alive part can not mutate the sequence.

pub mod error;
pub mod states;
pub mod types;

use self::{
    states::*,
    types::{
        AliveElementsPart, ParentSequenceRef, ParentSequenceRefMut, RangePart, RangePartImmut,
        RangePartMut,
    },
};

/// This type represents a part of a sequence.
/// it could be the alive elements or a custom
/// range of the sequence.
pub struct SequencePart<P, S> {
    part: P,
    parent_sequence: S,
    iter_index: usize,
}

/// Shared behavior between range part
/// and alive elements part.
pub trait SharedSequencePartBehavior<'a, T, I> {
    /// Returns the length of the sequence part.
    fn len(&self) -> usize;

    /// Checks if the sequence part is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a reference to the parent sequence of the sequence part.
    fn parent_sequence(&'a self) -> ParentSequenceRef<'a, T, I>;
}

impl<'a, T, I> AliveElementsPart<'a, T, I> {
    /// Create a new instance that represents 
    /// the alive elements part of a sequence.
    pub(super) fn new(parent_sequence: ParentSequenceRef<'a, T, I>) -> Self {
        Self {
            parent_sequence,
            part: AliveElements,
            iter_index: 0,
        }
    }

    /// Checks if the nth element is alive.
    pub fn nth_element_is_alive(&self, index: usize) -> bool {
        self.parent_sequence.nth_element_is_alive(index)
    }

    /// Returns the nth elements of the alive elements part.
    pub fn nth_element(&self, index: usize) -> Option<&T> {
        self.parent_sequence.nth_element_without_generation(index)
    }

    /// Returns the first element of the alive elements part.
    pub fn first_element(&self) -> Option<&T> {
        self.nth_element(0)
    }

    /// Returns the last element of the alive elements part.
    pub fn last_element(&self) -> Option<&T> {
        self.nth_element(self.len() - 1)
    }
}

impl<P> RangePart<P> {
    /// Checks if the element is in range.
    pub fn nth_element_is_in_range(&self, index: usize) -> bool {
        self.part.nth_element_is_in_range(index)
    }
}

impl<'a, T, I> RangePartImmut<'a, T, I> {
    /// Create a new instance that represents an immutable range of a sequence.
    pub(super) fn new_range(
        parent_sequence: ParentSequenceRef<'a, T, I>,
        start: usize,
        end: usize,
    ) -> Self {
        Self {
            part: Range::new(start, end),
            parent_sequence,
            iter_index: 0,
        }
    }

    /// Returns the nth element of the immutable range part.
    pub fn nth_element(&self, index: usize) -> Option<&T> {
        let index = index + self.part.start();

        if !self.nth_element_is_in_range(index) {
            return None;
        }

        self.parent_sequence.nth_element_without_generation(index)
    }

    /// Returns the first element of the immutable range part.
    pub fn first_element(&mut self) -> Option<&T> {
        self.nth_element(0)
    }

    /// Returns the last element of the immutable range part.
    pub fn last_element(&mut self) -> Option<&T> {
        self.nth_element(self.len() - 1)
    }
}

impl<'a, T, I> RangePartMut<'a, T, I> {
    /// Creates a new instance that represents a mutable range of a sequence.
    pub(super) fn new_range_mut(
        parent_sequence: ParentSequenceRefMut<'a, T, I>,
        start: usize,
        end: usize,
    ) -> Self {
        Self {
            parent_sequence,
            part: Range::new(start, end),
            iter_index: 0,
        }
    }

    /// Returns the nth element of the mutable range part.
    pub fn nth_element(&mut self, index: usize) -> Option<&T> {
        let index = index + self.part.start();

        if !self.nth_element_is_in_range(index) {
            return None;
        }

        Some(self.parent_sequence.nth_element(index))
    }

    /// Returns the first element of the mutable range part.
    pub fn first_element(&mut self) -> Option<&T> {
        self.nth_element(0)
    }

    /// Returns the last element of the mutable range part.
    pub fn last_element(&mut self) -> Option<&T> {
        self.nth_element(self.len() - 1)
    }
}

impl<'a, T, I> SharedSequencePartBehavior<'a, T, I> for AliveElementsPart<'a, T, I> {
    fn len(&self) -> usize {
        self.parent_sequence.alive_elements_len()
    }

    fn parent_sequence(&self) -> ParentSequenceRef<'a, T, I> {
        self.parent_sequence
    }
}

impl<'a, T, I> SharedSequencePartBehavior<'a, T, I> for RangePartImmut<'a, T, I> {
    fn len(&self) -> usize {
        self.part.end() - self.part.start()
    }

    fn parent_sequence(&self) -> ParentSequenceRef<'a, T, I> {
        self.parent_sequence
    }
}

impl<'a, T, I> SharedSequencePartBehavior<'a, T, I> for RangePartMut<'a, T, I> {
    fn len(&self) -> usize {
        self.part.end() - self.part.start()
    }

    fn parent_sequence(&'a self) -> ParentSequenceRef<'a, T, I> {
        self.parent_sequence
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

impl<'a, T: Clone, I> Iterator for RangePartImmut<'a, T, I> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let iter_index = self.iter_index;

        if iter_index == self.part.end() {
            self.iter_index = 0;
            return None;
        }

        self.iter_index += 1;
        self.nth_element(iter_index).cloned()
    }
}

impl<'a, T: Clone, I> Iterator for RangePartMut<'a, T, I> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let iter_index = self.iter_index;

        if iter_index == self.part.end() {
            self.iter_index = 0;
            return None;
        }

        self.iter_index += 1;
        self.nth_element(iter_index).cloned()
    }
}
