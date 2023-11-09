//! This module defines the states of the Sequence type

use super::{types::TransitionFunction, Sequence};

use crate::sequence_part::{states::AliveElements, SequencePart};

/// A type that represents when the state
/// has no initial elements
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WithoutInitialElements;

/// A type that represents when the state
/// has initial elements
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WithoutTransitionFunction;

/// A type that represents when the state
/// has transition function
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WithTransitionFunction<T, I>(TransitionFunction<T, I>);

impl<T, I> WithTransitionFunction<T, I> {
    /// Create new instance
    pub fn new(trans_func: TransitionFunction<T, I>) -> Self {
        Self(trans_func)
    }

    /// Runs the transition function
    pub fn run(
        &self,
        alive_elements_part: SequencePart<
            AliveElements,
            &Sequence<T, I, WithTransitionFunction<T, I>>,
        >,
        current_element_index: usize,
    ) -> T {
        (self.0)(alive_elements_part, current_element_index)
    }
}
