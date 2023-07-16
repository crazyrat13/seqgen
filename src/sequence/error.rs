//! This module defines errors

use std::fmt::Debug;

pub enum RangeErrorKind {
    InvalidRange,
}

impl Debug for RangeErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRange => write!(f, "InvalidRange: start of range is greater than its end"),
        }
    }
}

pub struct RangeError {
    kind: RangeErrorKind,
}

impl RangeError {
    pub fn new(kind: RangeErrorKind) -> Self {
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
