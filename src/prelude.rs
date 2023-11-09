//! Prelude for the library

use crate::sequence::{states::*, types::*};

pub use crate::{
    sequence::{Sequence, SharedSequenceBehavior},
    sequence_part::{error::*, SequencePart, SharedSequencePartBehavior},
};

/// Create new sequence that do not require initial elements
pub fn seq_without_initial_elements<T>(
    transition_function: TransitionFunction<T, WithoutInitialElements>,
) -> Sequence<T, WithoutInitialElements, WithTransitionFunction<T, WithoutInitialElements>> {
    Sequence::new().transition_function(transition_function)
}

/// Create new sequence that do require initial elements
pub fn seq_with_initial_elements<T>(
    initial_elements: Vec<T>,
    transition_function: TransitionFunction<T, WithInitialElements>,
) -> Sequence<T, WithInitialElements, WithTransitionFunction<T, WithInitialElements>> {
    Sequence::new()
        .initial_elements(initial_elements)
        .transition_function(transition_function)
}
