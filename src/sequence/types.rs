//! This module defines type aliases for sequence

use crate::{sequence_part::{states::AliveElements, SequencePart}, prelude::WithTransitionFunction};

use super::Sequence;

/// A type that represents the transition function
pub type TransitionFunction<T, I> = fn(
    SequencePart<AliveElements, &Sequence<T, I, WithTransitionFunction<T, I>>>,
    usize,
) -> T;
