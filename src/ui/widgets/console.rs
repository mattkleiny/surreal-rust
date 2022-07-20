//! An in-game console for the user to interact with.

use std::ops::RangeInclusive;

use egui::{panel::TopBottomSide, Id, InnerResponse};

/// An in-game console that can be rendered on top of the game's UI.
///
/// The console displays the most recent log messages and permits
/// basic command execution via a `ConsoleInterpreter` implementation.
#[must_use = "You should call .show()"]
pub struct DropDownConsole<'a> {
  id: Id,
  side: TopBottomSide,
  height_range: RangeInclusive<f32>,
  interpreter: Option<&'a mut dyn ConsoleInterpreter>,
}

/// An interpreter allows a [`DropDownConsole`] to respond to user commands.
pub trait ConsoleInterpreter {
  fn interpret(&mut self, command: String) -> Option<()>;
}

impl<'a> DropDownConsole<'a> {
  pub fn top(id_source: impl std::hash::Hash) -> Self {
    Self::new(TopBottomSide::Top, id_source)
  }

  pub fn bottom(id_source: impl std::hash::Hash) -> Self {
    Self::new(TopBottomSide::Bottom, id_source)
  }

  pub fn new(side: TopBottomSide, id_source: impl std::hash::Hash) -> Self {
    Self {
      side,
      id: Id::new(id_source),
      height_range: 20.0..=f32::INFINITY,
      interpreter: None,
    }
  }

  pub fn min_height(mut self, min_height: f32) -> Self {
    self.height_range = min_height..=(*self.height_range.end());
    self
  }

  pub fn max_height(mut self, max_height: f32) -> Self {
    self.height_range = (*self.height_range.start())..=max_height;
    self
  }

  pub fn height_range(mut self, height_range: RangeInclusive<f32>) -> Self {
    self.height_range = height_range;
    self
  }

  pub fn interpreter(mut self, interpreter: &'a mut dyn ConsoleInterpreter) -> Self {
    self.interpreter = Some(interpreter);
    self
  }

  pub fn show(self) -> InnerResponse<()> {
    todo!()
  }
}
