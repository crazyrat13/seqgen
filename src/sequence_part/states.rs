//! This module defines states that the SequencePart could be one of

/// A type that represents the state of SequencePart
/// when its used to represents the alive elements of the sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AliveElements;

// A type that represents the state of SequencePart
/// when its used to represents a range of the sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Range {
    start: usize,
    end: usize,
}

impl Range {
    /// Create new instance.
    pub(super) fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Returns the start of the range.
    pub(super) fn start(&self) -> usize {
        self.start
    }

    /// Returns the end of the range.
    pub(super) fn end(&self) -> usize {
        self.end
    }

    /// Checks if an element is in range.
    pub fn nth_element_is_in_range(&self, index: usize) -> bool {
        index >= self.start() && index < self.end()
    }
}
