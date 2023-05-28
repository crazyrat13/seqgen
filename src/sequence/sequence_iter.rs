//! This module defines the sequence iterator type

use super::Sequence;

/// SequenceIter: an iterator over the sequence
pub struct SequenceIter<'a, T, S> {
    sequence: Sequence<'a, T, S>,
    iter_index: usize,
}

impl<'a, T, S> SequenceIter<'a, T, S> {
    /// Creates a new sequence iterator
    pub(super) fn new(sequence: Sequence<'a, T, S>) -> Self {
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
