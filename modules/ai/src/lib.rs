//! AI components for games.
//!
//! Decision making, pathfinding, automata and tooling.

/// An 'automata' is a thinking object that can create decisions.
pub trait Automata<M> {
  fn think(&mut self, memory: &mut M, delta_time: f32);
}

/// A 'brain' is a collection of [`Automata`] that can be ticked as a unit.
pub struct Brain<M = ()> {
  automata: Vec<Box<dyn Automata<M>>>,
}

impl<M> Brain<M> {
  /// Applies one step to all [`Automata`] in the brain.s
  pub fn think(&mut self, memory: &mut M, delta_time: f32) {
    for automata in &mut self.automata {
      automata.think(memory, delta_time);
    }
  }
}

/// A simple finite state machine [`Automata`].
///
/// The state machine is composed of a stack of [`State`] instances, each of
/// which can be pushed or popped from the stack. The top state is the current
/// state.
#[derive(Default)]
pub struct StateMachine<M = ()> {
  states: Vec<Box<dyn State<M>>>,
  current_state: Option<usize>,
}

/// A single state in a [`StateMachine`].
pub trait State<M> {
  /// Updates this state, returning a [`StateTransition`] that indicates what
  /// should happen next.
  fn think(&mut self, memory: &mut M, delta_time: f32) -> StateTransition<M>;
}

/// Transitions for a [`State`] in a [`StateMachine`].
pub enum StateTransition<M> {
  /// No transition.
  Continue,
  /// Removes this [`State`], transitions to the previous [`State`] if it
  /// exists.
  Pop,
  /// Moves to the next [`State`], retaining the old one to return to.
  Push(Box<dyn State<M>>),
  /// Replaces the current [`State`] with a new one.
  Replace(Box<dyn State<M>>),
}

impl<M> StateMachine<M> {
  /// Adds a new [`State`] to the state machine, retaining the old one to return
  /// to.
  pub fn push(&mut self, state: Box<dyn State<M>>) {
    self.states.push(state);
    self.current_state = Some(self.states.len() - 1);
  }

  /// Removes the current [`State`], transitioning to the previous [`State`] if
  /// it exists.
  pub fn pop(&mut self) {
    self.states.pop();
    self.current_state = self.states.len().checked_sub(1);
  }

  /// Replaces the current [`State`] with a new one.
  pub fn replace(&mut self, state: Box<dyn State<M>>) {
    self.states.pop();
    self.states.push(state);
    self.current_state = Some(self.states.len() - 1);
  }
}

impl<M> Automata<M> for StateMachine<M> {
  /// Updates the current [`State`] in the state machine and applies any
  /// [`StateTransition`]s.
  fn think(&mut self, memory: &mut M, delta_time: f32) {
    if let Some(state_index) = self.current_state {
      if let Some(state) = self.states.get_mut(state_index) {
        match state.think(memory, delta_time) {
          StateTransition::Continue => {} // no-op
          StateTransition::Pop => self.pop(),
          StateTransition::Push(new_state) => self.push(new_state),
          StateTransition::Replace(new_state) => self.replace(new_state),
        };
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn state_machine_should_update_top_most_state() {
    struct TestState {
      counter: usize,
    }

    impl State<()> for TestState {
      fn think(&mut self, _memory: &mut (), _delta_time: f32) -> StateTransition<()> {
        println!("Thinking! {}", self.counter);
        StateTransition::Replace(Box::new(TestState {
          counter: self.counter + 1,
        }))
      }
    }

    let mut state_machine = StateMachine::default();

    state_machine.push(Box::new(TestState { counter: 1 }));

    state_machine.think(&mut (), 0.0);
    state_machine.think(&mut (), 0.0);
    state_machine.think(&mut (), 0.0);
    state_machine.think(&mut (), 0.0);
    state_machine.think(&mut (), 0.0);
    state_machine.think(&mut (), 0.0);

    assert_eq!(1, state_machine.states.len());
  }
}
