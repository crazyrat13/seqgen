//! This module defines type aliases for sequence part

use super::{
    error::RangeError,
    states::{AliveElements, Range},
    SequencePart,
};

use crate::sequence::{states::WithTransitionFunction, Sequence};

/// An immutable reference to the parent sequence.
pub type ParentSequenceRef<'a, T, I> = &'a Sequence<T, I, WithTransitionFunction<T, I>>;

/// A mutable reference to the parent sequence.
pub type ParentSequenceRefMut<'a, T, I> = &'a mut Sequence<T, I, WithTransitionFunction<T, I>>;

/// Alive elements part.
pub type AliveElementsPart<'a, T, I> = SequencePart<AliveElements, ParentSequenceRef<'a, T, I>>;

/// Range part type.
pub type RangePart<'a, T, I> = SequencePart<Range, ParentSequenceRefMut<'a, T, I>>;

/// Range result that is returned when creating ranges.
pub type RangeResult<'a, T, I> = Result<RangePart<'a, T, I>, RangeError>;
