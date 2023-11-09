//! This module defines type aliases for sequence

use super::{states::WithTransitionFunction, Sequence};

use crate::sequence_part::{states::AliveElements, SequencePart};

/// A type that represents the transition function.
/// The first parameter is the alive element part of
/// the sequence, and the second is the index of the currently generated element
pub type TransitionFunction<T, I> =
    fn(SequencePart<AliveElements, &Sequence<T, I, WithTransitionFunction<T, I>>>, usize) -> T;
