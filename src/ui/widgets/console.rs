//! An in-game console for the user to interact with.

use std::ops::RangeInclusive;

use egui::{panel::TopBottomSide, Frame, Id, InnerResponse};

/// An in-game console that can be rendered on top of the game's UI.
///
/// The console displays the most recent log messages and permits
/// basic command execution via a `ConsoleInterpreter` implementation.
#[must_use = "You should call .show()"]
pub struct InGameConsole<'a> {
  side: TopBottomSide,
  id: Id,
  frame: Option<Frame>,
  resizable: bool,
  default_height: Option<f32>,
  height_range: RangeInclusive<f32>,
  interpreter: Option<&'a dyn ConsoleInterpreter>,
}

/// An interpreter allows an `InGameConsole` to respond to user commands.
pub trait ConsoleInterpreter {}

impl<'a> InGameConsole<'a> {
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
      frame: None,
      resizable: false,
      default_height: None,
      height_range: 20.0..=f32::INFINITY,
      interpreter: None,
    }
  }

  pub fn resizable(mut self, resizable: bool) -> Self {
    self.resizable = resizable;
    self
  }

  pub fn default_height(mut self, default_height: f32) -> Self {
    self.default_height = Some(default_height);
    self
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

  pub fn frame(mut self, frame: Frame) -> Self {
    self.frame = Some(frame);
    self
  }

  pub fn interpreter(mut self, interpreter: &'a dyn ConsoleInterpreter) -> Self {
    self.interpreter = Some(interpreter);
    self
  }

  pub fn show(self) -> InnerResponse<()> {
    todo!()
  }
}
