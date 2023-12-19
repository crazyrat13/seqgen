//! This module defines type aliases for sequence parts

use super::{
    error::RangeError,
    states::{AliveElements, Range},
    SequencePart,
};

use crate::sequence::{states::WithTransitionFunction, Sequence};

/// An immutable reference to the parent sequence.
pub type ParentSequenceRef<'a, T, I> = &'a Sequence<T, I, WithTransitionFunction<T, I>>;

/// A mutable reference to the parent sequence.
pub type ParentSequenceRefMut<'a, T, I> =
    &'a mut Sequence<T, I, WithTransitionFunction<T, I>>;

/// Alive elements part.
pub type AliveElementsPart<'a, T, I> =
    SequencePart<AliveElements, ParentSequenceRef<'a, T, I>>;

/// Generic range part.
pub type RangePart<P> = SequencePart<Range, P>;

/// Immutable range part type.
pub type RangePartImmut<'a, T, I> = RangePart<ParentSequenceRef<'a, T, I>>;

/// Mutable range part type.
pub type RangePartMut<'a, T, I> = RangePart<ParentSequenceRefMut<'a, T, I>>;

/// Range part result that is returned when creating immutable ranges.
pub type RangePartImmutResult<'a, T, I> = Result<RangePartImmut<'a, T, I>, RangeError>;

/// Range part result that is returned when creating mutable ranges.
pub type RangePartMutResult<'a, T, I> = Result<RangePartMut<'a, T, I>, RangeError>;
