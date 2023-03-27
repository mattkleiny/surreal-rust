//! A module for roguelike games.

pub type TurnNumber = u32;

/// An actor that can execute commands.
pub trait Actor {
  /// Returns the next command to execute.
  fn act(&mut self, _context: &TurnContext) -> Box<dyn Command>;
}

/// Context for a turn.
pub struct TurnContext {
  /// The number of the turn.
  pub turn: TurnNumber,
}

/// A command that can be executed by an actor.
pub trait Command {
  /// Executes the command agains the given .
  fn execute(&mut self) -> CommandResult;
}

/// The result of a command execution.
pub enum CommandResult {
  Success,
  Failure(String),
  Alternative(Box<dyn Command>),
}

/// An oberver that is notified when a turn starts or ends.
pub trait TurnObserver {
  /// Called when a turn starts.
  fn on_turn_started(&mut self, _context: &TurnContext) {}

  /// Called when a turn ends.
  fn on_turn_ended(&mut self, _context: &TurnContext) {}
}

/// Manages the turns of the game.
pub struct TurnManager {
  current_turn: TurnNumber,
  actors: Vec<Box<dyn Actor>>,
  observers: Vec<Box<dyn TurnObserver>>,
}

impl Default for TurnManager {
  fn default() -> Self {
    Self::new()
  }
}

impl TurnManager {
  /// Creates a new turn manager.
  pub fn new() -> Self {
    Self {
      current_turn: 1,
      actors: Vec::new(),
      observers: Vec::new(),
    }
  }

  /// Adds an actor that will be updated on each turn.
  pub fn add_actor(&mut self, actor: Box<dyn Actor>) {
    self.actors.push(actor);
  }

  /// Adds an oberver that will be notified when a turn starts or ends.
  pub fn add_observer(&mut self, observer: Box<dyn TurnObserver>) {
    self.observers.push(observer);
  }

  /// Executes the next turn.
  pub fn next_turn(&mut self) {
    let context = TurnContext {
      turn: self.current_turn,
    };

    for observer in &mut self.observers {
      observer.on_turn_started(&context);
    }

    // execute commands
    for actor in &mut self.actors {
      let mut command = actor.act(&context);

      while let CommandResult::Alternative(alternative) = command.execute() {
        command = alternative;
      }
    }

    for observer in &mut self.observers {
      observer.on_turn_ended(&context);
    }

    self.current_turn += 1;
  }
}

#[cfg(test)]
mod tests {
  use surreal::maths::{ivec2, IVec2};

  use super::*;

  #[derive(Default)]
  struct TestActor {
    _position: IVec2,
  }

  impl Actor for TestActor {
    fn act(&mut self, _context: &TurnContext) -> Box<dyn Command> {
      Box::new(MoveCommand::new(ivec2(-1, 0)))
    }
  }

  struct MoveCommand {
    _direction: IVec2,
  }

  impl MoveCommand {
    fn new(direction: IVec2) -> Self {
      Self {
        _direction: direction,
      }
    }
  }

  impl Command for MoveCommand {
    fn execute(&mut self) -> CommandResult {
      CommandResult::Success
    }
  }

  #[test]
  fn turn_manager_should_execute_basic_loop() {
    let mut turn_manager = TurnManager::new();

    turn_manager.add_actor(Box::<TestActor>::default());
    turn_manager.add_actor(Box::<TestActor>::default());

    turn_manager.next_turn();
  }
}
