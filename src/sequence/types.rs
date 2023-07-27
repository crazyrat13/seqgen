//! This module defines type aliases

use crate::sequence_part::{states::AliveElements, SequencePart};

pub type TransitionFunction<T> = fn(SequencePart<'_, T, AliveElements>, usize) -> T;
