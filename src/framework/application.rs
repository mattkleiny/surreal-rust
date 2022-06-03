//! A simple application layer for the engine.
//!
//! Applications are entry points for more complex engine usages, and
//! form the core of the engine and foundation for event plumbing.

use crate::{maths::Vector2, platform::Platform};

/// Entry point for a Surreal-based application.
pub struct Application<P: Platform> {
  host: P::Host,
}

/// Represents a listener that can receive events from an application.
pub trait ApplicationListener<P: Platform> {
  /// Invoked when the application should tick.
  fn tick(&self);

  /// Invoked when the application should render.
  fn render(&self);
}

impl<P: Platform> Application<P> {
  /// Creates a new application on the given platform.
  pub fn new(platform: P) -> Self {
    Self {
      host: platform.create_host(),
    }
  }

  /// Runs the application with the given main body.
  pub fn run(&mut self, _listener: impl ApplicationListener<P> + 'static) {
    // TODO: handle listener invocations
  }
}

// platform events
pub struct PlatformTickEvent();
pub struct PlatformRenderEvent();
pub struct PlatformResizedEvent(pub usize, pub usize);
pub struct PlatformClosedEvent();

// input events
pub struct KeyPressdEvent(pub crate::input::Key);
pub struct KeyReleasedEvent(pub crate::input::Key);
pub struct MouseMovedEvent(pub Vector2<usize>);
pub struct MouseScrolledEvent(pub Vector2<usize>);
pub struct MousePressedEvent(pub crate::input::MouseButton);
pub struct MouseReleasedEvent(pub crate::input::MouseButton);
