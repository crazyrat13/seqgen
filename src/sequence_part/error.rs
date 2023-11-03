//! This module defines errors for sequence part

use std::fmt::Debug;

/// Range error kind
pub enum RangeErrorKind {
    /// Represents an invalid range (start of range greater than its end)
    InvalidRange {
        /// Start of the range
        start: usize,
        /// End of the range
        end: usize,
    },
}

impl Debug for RangeErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRange { start, end } => write!(
                f,
                "InvalidRange: Start of range is greater than its end: start: {start} > end: {end}",
            ),
        }
    }
}

/// Range error happens when we try to get an invalid range from a sequence using Sequence::range.
/// Example of this error could be trying to get a range where its end is less then its start
pub struct RangeError {
    /// The kind of range error
    pub kind: RangeErrorKind,
}

impl RangeError {
    /// creates new range error
    pub(crate) fn new(kind: RangeErrorKind) -> Self {
        Self { kind }
    }
}

impl Debug for RangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RangeError")
            .field("kind", &self.kind)
            .finish()
    }
}
