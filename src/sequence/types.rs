//! This module defines type aliases

use super::sequence_part::states::AliveElements;
use super::sequence_part::SequencePart;

pub type TransitionFunction<T> = fn(SequencePart<'_, T, AliveElements>, usize) -> T;
