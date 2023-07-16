//! This module defines states that the SequencePart could be one of

/// A type that represents the state of SequencePart
/// when its used to represents the alive elements of the sequence
pub struct AliveElements;

// A type that represents the state of SequencePart
/// when its used to represents a range of the sequence
pub struct Range {
    start: usize,
    end: usize,
}

impl Range {
    /// Create new instance
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Returns the start of the range
    pub fn start(&self) -> usize {
        self.start
    }

    /// Returns the end of the range
    pub fn end(&self) -> usize {
        self.end
    }
}
