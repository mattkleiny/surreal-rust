//! General utilities.

pub use events::*;
pub use fibers::*;
pub use languages::*;
pub use memory::*;
pub use rle::*;
pub use timing::*;

mod events;
mod fibers;
mod languages;
mod memory;
mod rle;
mod timing;

/// A super simple and lightweight state machine.
/// Events are published via `on_changed` when the state transitions.
pub struct FSM<S> {
  pub current_state: S,
  pub previous_state: S,
  pub on_changed: Event<(S, S)>,
}

impl<S> FSM<S> where S: Copy + Eq {
  pub fn new(initial_state: S) -> Self {
    Self {
      current_state: initial_state,
      previous_state: initial_state,
      on_changed: Event::new(),
    }
  }

  /// Swaps to the given new state.
  pub fn swap(&mut self, new_state: S) -> bool {
    if self.current_state != new_state {
      self.previous_state = self.current_state;
      self.current_state = new_state;

      self.on_changed.publish(&(self.previous_state, self.current_state));

      return true;
    }

    false
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_manage_state_transitions() {
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum State { Idle, Wandering }

    let mut fsm = FSM::new(State::Idle);

    fsm.on_changed += |(old_state, new_state)| {
      println!("Changed from {:?} to {:?}", old_state, new_state);
    };

    fsm.swap(State::Wandering);
    fsm.swap(State::Wandering);
    fsm.swap(State::Wandering);
    fsm.swap(State::Idle);
  }
}