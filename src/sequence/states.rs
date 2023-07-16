//! This module defines the states of the Sequence type

use super::{
    sequence_part::{states::AliveElements, SequencePart},
    types::TransitionFunction,
};

/// A type that represents when the state
/// has no initial elements
pub struct WithoutInitialElements;

/// A type that represents when the state
/// has initial elements
pub struct WithInitialElements(usize);

impl WithInitialElements {
    /// Create new instance
    pub fn new(initial_elements_len: usize) -> Self {
        Self(initial_elements_len)
    }

    /// Returns the length of the initial elements
    pub fn len(&self) -> usize {
        self.0
    }

    /// Checks if the initial elements are empty
    pub fn is_empty(&self) -> bool {
        if self.0 != 0 {
            return false;
        }

        true
    }
}

/// A type that represents when the state
/// has no transition function
pub struct WithoutTransitionFunction;

/// A type that represents when the state
/// has transition function
pub struct WithTransitionFunction<T>(TransitionFunction<T>);

impl<T> WithTransitionFunction<T> {
    /// Create new instance
    pub fn new(trans_func: TransitionFunction<T>) -> Self {
        Self(trans_func)
    }

    /// Runs the transition function
    pub fn run(
        &self,
        alive_elements_part: SequencePart<'_, T, AliveElements>,
        current_element_index: usize,
    ) -> T {
        (self.0)(alive_elements_part, current_element_index)
    }
}
