//! This module defines errors

use std::fmt::Debug;

/// Range error kind
pub enum RangeErrorKind {
    /// Represents an invalid range (start of range greater than its end)
    InvalidRange,
}

impl Debug for RangeErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRange => write!(f, "InvalidRange: start of range is greater than its end"),
        }
    }
}

/// Range error
pub struct RangeError {
    kind: RangeErrorKind,
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
