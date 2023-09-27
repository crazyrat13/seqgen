//! This module defines type aliases

use crate::sequence_part::{states::AliveElements, SequencePart};

pub type TransitionFunction<T, I> = fn(SequencePart<'_, T, I, AliveElements>, usize) -> T;
