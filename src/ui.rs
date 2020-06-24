//! A lightweight UI widget framework for Surreal.
//!
//! This is a simple widget-based immediate mode framework that works directly
//! on top of the rendering engine.
//!
//! It's designed for editing tools and simple layouts.

use crate::maths::ScreenRect;
use crate::platform::GameTime;

pub struct ScissorRect(ScreenRect<u32>);

/// A canvas that can be rendered directly onto the screen.
/// A canvas consists of many `Widget`s in `Layout`s.
pub struct Canvas {
  rect: Option<ScissorRect>,
  widgets: Vec<Box<dyn Widget>>,
}

impl Canvas {
  pub fn new() -> Self {
    Self {
      rect: None,
      widgets: Vec::new(),
    }
  }

  pub fn update(&mut self, time: GameTime) {
    for widget in self.widgets.iter_mut() {
      widget.update(time);
    }
  }

  pub fn draw(&self, time: GameTime, renderer: &dyn CanvasRenderer) {
    for widget in self.widgets.iter() {
      widget.draw(time, renderer);
    }
  }
}

/// A component capable of rendering `Canvas` objects.
pub trait CanvasRenderer {}

/// A widget that can be rendered by a canvas.
///
/// Widgets are updated every frame and are responsible for their own rendering and control.
pub trait Widget {
  fn compute_layout(&self) -> ScissorRect;
  fn update(&mut self, time: GameTime);
  fn draw(&self, time: GameTime, renderer: &dyn CanvasRenderer);
}

/// A vertically stacked widget group.
struct VerticalLayout;

impl Widget for VerticalLayout {
  fn compute_layout(&self) -> ScissorRect {
    unimplemented!()
  }

  fn update(&mut self, time: GameTime) {
    unimplemented!()
  }

  fn draw(&self, time: GameTime, renderer: &dyn CanvasRenderer) {
    unimplemented!()
  }
}
