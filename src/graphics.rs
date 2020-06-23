//! A lightweight and fast cross-platform graphics engine.
//!
//! This implementation is designed to be portable across platforms through consumption of a standard graphics API
//! across all of those platforms, as opposed to offering different APIs for different platforms (ala gfx-hal).
//!
//! Whilst more directly coupled than other providers, this implementation is simple, direct and fast. It is designed
//! to account for the majority use case as opposed to all possibilities and to do it well, as opposed to solving
//! the general case and doing it poorly.
//!
//! This device, whilst not strictly precluding 3d development, is primarily focused towards 2d development, and a lot
//! of the auxiliary libraries and utilities are designed to be fast in 2-space.

pub use colors::*;

mod colors;
