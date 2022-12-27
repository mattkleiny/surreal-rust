//! Finite state machine utilities.

use std::{collections::HashMap, hash::Hash};

/// A simple finite state machine with unique data per state, `T`.
pub struct FSM<S, T = ()> {
  state: S,
  data: HashMap<S, T>,
}

impl<S: Default, T> Default for FSM<S, T> {
  fn default() -> Self {
    Self {
      state: S::default(),
      data: HashMap::new(),
    }
  }
}

impl<S: Hash + Eq, T> FSM<S, T> {
  /// Initializes the machine with the given state and data.
  pub fn with(mut self, state: S, data: T) -> Self {
    self.data.insert(state, data);
    self
  }

  /// Sets the current state to the given state, if possible.
  pub fn set_state(&mut self, state: S) -> bool {
    if self.state != state {
      self.state = state;
      true
    } else {
      false
    }
  }

  /// Borrows the current state.
  pub fn current_state(&self) -> &S {
    &self.state
  }

  /// Borrows the current data of the current state.
  pub fn current_data(&self) -> Option<&T> {
    if let Some(data) = self.data.get(&self.state) {
      Some(data)
    } else {
      None
    }
  }

  /// Mutably borrows the current data of the current state.
  pub fn current_data_mut(&mut self) -> Option<&mut T> {
    if let Some(data) = self.data.get_mut(&self.state) {
      Some(data)
    } else {
      None
    }
  }
}
