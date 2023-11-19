//! This module defines type aliases for sequence part

use super::{
    error::RangeError,
    states::{AliveElements, Range},
    SequencePart,
};

use crate::sequence::{states::WithTransitionFunction, Sequence};

/// An immutable reference to the parent sequence.
pub(crate) type ParentSequenceRef<'a, T, I> = &'a Sequence<T, I, WithTransitionFunction<T, I>>;

/// A mutable reference to the parent sequence.
pub(crate) type ParentSequenceRefMut<'a, T, I> =
    &'a mut Sequence<T, I, WithTransitionFunction<T, I>>;

/// Alive elements part.
pub(crate) type AliveElementsPart<'a, T, I> =
    SequencePart<AliveElements, ParentSequenceRef<'a, T, I>>;

pub(crate) type RangePart<P> = SequencePart<Range, P>;

/// Immutable range part type.
pub(crate) type RangePartImmut<'a, T, I> = RangePart<ParentSequenceRef<'a, T, I>>;

/// Mutable range part type.
pub(crate) type RangePartMut<'a, T, I> = RangePart<ParentSequenceRefMut<'a, T, I>>;

/// Range result that is returned when creating immutable ranges.
pub(crate) type RangeResult<'a, T, I> = Result<RangePartImmut<'a, T, I>, RangeError>;

/// Range result that is returned when creating mutable ranges.
pub(crate) type RangeMutResult<'a, T, I> = Result<RangePartMut<'a, T, I>, RangeError>;
