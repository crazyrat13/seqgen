//! This module defines the states of the Sequence type

use super::types::TransitionFunction;

use crate::sequence_part::types::AliveElementsPart;

/// A type that represents when the sequence
/// has no initial elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WithoutInitialElements;

/// A type that represents when the sequence
/// has initial elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WithInitialElements(usize);

impl WithInitialElements {
    /// Create new instance.
    pub fn new(initial_elements_len: usize) -> Self {
        Self(initial_elements_len)
    }

    /// Returns the length of the initial elements.
    pub fn len(&self) -> usize {
        self.0
    }

    /// Checks if the initial elements are empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// A type that represents when the sequence
/// has no transition function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WithoutTransitionFunction;

/// A type that represents when the sequence
/// has transition function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WithTransitionFunction<T, I>(TransitionFunction<T, I>);

impl<T, I> WithTransitionFunction<T, I> {
    /// Create new instance.
    pub(super) fn new(trans_func: TransitionFunction<T, I>) -> Self {
        Self(trans_func)
    }

    /// Returns the transition function.
    pub(super) fn transition_function(&self) -> TransitionFunction<T, I> {
        self.0
    }

    /// Runs the transition function.
    pub(super) fn run(
        &self,
        alive_elements_part: AliveElementsPart<'_, T, I>,
        current_element_index: usize,
    ) -> T {
        (self.transition_function())(alive_elements_part, current_element_index)
    }
}
