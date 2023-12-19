//! This module defines type aliases for the sequence module

use crate::sequence_part::types::AliveElementsPart;

/// A type that represents the transition function.
/// The first parameter is the alive elements part of
/// the sequence, and the second is the index of the
/// current element in generation.
pub type TransitionFunction<T, I> = fn(AliveElementsPart<'_, T, I>, usize) -> T;
