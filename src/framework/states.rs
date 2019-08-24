//! State management and abstractions.

/// Abstracts over a state in the state manager.
pub trait State<C> {
  fn input(&mut self, context: C);
  fn update(&mut self, context: C);
  fn draw(&mut self, context: C);
}

/// Manages a set of states.
pub struct StateManager<C> {
  states: Vec<Box<dyn State<C>>>
}

impl<C> StateManager<C> {
  pub fn new() -> Self {
    Self { states: Vec::new() }
  }

  /// Pushes a new state onto the manager.
  pub fn push<S: 'static + State<C>>(&mut self, state: S) {
    self.states.push(Box::new(state));
  }

  /// Pops the top-most state from the manager.
  pub fn pop(&mut self) {
    self.states.pop();
  }

  /// Invokes 'input' on the top-most state in the manager.
  pub fn input(&mut self, context: C) {
    match self.states.last_mut() {
      Some(state) => state.input(context),
      None => {}
    }
  }

  /// Invokes 'update' on the top-most state in the manager.
  pub fn update(&mut self, context: C) {
    match self.states.last_mut() {
      Some(state) => state.update(context),
      None => {}
    }
  }

  /// Invokes 'draw' on the top-most state in the manager.
  pub fn draw(&mut self, context: C) {
    match self.states.last_mut() {
      Some(state) => state.draw(context),
      None => {}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct TestState;

  impl State<f64> for TestState {
    fn input(&mut self, _delta_time: f64) {}
    fn update(&mut self, _delta_time: f64) {}
    fn draw(&mut self, _delta_time: f64) {}
  }

  #[test]
  fn it_should_push_and_pop_states() {
    let mut manager = StateManager::new();
    let delta_time = 0.16;

    manager.push(TestState);
    manager.input(delta_time);
    manager.update(delta_time);
    manager.draw(delta_time);
    manager.pop();
  }
}