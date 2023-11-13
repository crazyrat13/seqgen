//! This module defines errors for sequence part

use std::{
    error::Error,
    fmt::{Debug, Display},
};

/// Range error happens when we try to get an invalid range from a sequence using Sequence::range.
/// Example of this error could be trying to get a range where its end is less then its start
pub enum RangeError {
    /// Represents an invalid range (start of range greater than its end)
    InvalidRange {
        /// Start of the range
        start: usize,
        /// End of the range
        end: usize,
    },
}

impl Debug for RangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RangeError::InvalidRange { start, end } => f
                .debug_struct("InvalidRange")
                .field("start", start)
                .field("end", end)
                .finish(),
        }
    }
}

impl Display for RangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RangeError::InvalidRange { start, end } => {
                write!(f, "Range start ({start}) is greater than its end ({end}).")
            }
        }
    }
}

impl Error for RangeError {}
