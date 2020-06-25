//! A lightweight and fast cross-platform graphics engine.
//!
//! This implementation is designed to be portable across platforms through consumption of a
//! standard graphics API across all of those platforms, as opposed to offering different APIs
//! for different platforms (ala gfx-hal).
//!
//! Whilst more directly coupled than other providers, this implementation is simple, direct and
//! fast. It is designed to account for the majority use case as opposed to all possibilities and
//! to do it well, as opposed to solving the general case and doing it poorly.
//!
//! This implementation, whilst not strictly precluding 3d development, is primarily focused towards
//! 2d development, and a lot of the auxiliary libraries and utilities are designed to be fast in 2d.

pub use colors::*;
pub use sprites::*;

mod colors;
mod sprites;

/// Permits interacting with the graphics sub-system.
pub trait GraphicsDevice {
  fn clear(&mut self, color: Color);
}

pub trait Renderable {
  fn render(&self, device: &mut impl GraphicsDevice);
}
