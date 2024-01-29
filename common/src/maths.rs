//! Mathematical utilities.

pub use angles::*;
pub use cameras::*;
pub use curves::*;
pub use geometry::*;
pub use hex::*;
pub use lerp::*;
pub use linear::*;
pub use neighbours::*;
pub use paths::*;
pub use random::*;
pub use ranges::*;
pub use rectangles::*;
pub use size::*;
pub use shapes::*;
pub use splines::*;
pub use time::*;
pub use viewports::*;

mod angles;
mod cameras;
mod curves;
mod geometry;
mod hex;
mod lerp;
mod linear;
mod neighbours;
mod paths;
mod random;
mod ranges;
mod rectangles;
mod shapes;
mod size;
mod splines;
mod time;
mod viewports;

/// A globally unique identifier.
pub type Guid = uuid::Uuid;

/// Allows approximate equality checks between values.
pub trait ApproxEq<T = Self> {
  const EPSILON: T;

  /// Determines whether two values are equal within a given delta.
  fn approx_eq_delta(&self, other: T, delta: T) -> bool;

  /// Determines whether two values are approximately equal.
  #[inline]
  fn approx_eq(&self, other: T) -> bool {
    self.approx_eq_delta(other, Self::EPSILON)
  }
}

macro_rules! impl_approx_eq {
  ($type:ty) => {
    impl ApproxEq for $type {
      const EPSILON: $type = 0.00001;

      #[inline]
      fn approx_eq_delta(&self, other: Self, delta: $type) -> bool {
        (other - self).abs() < delta
      }
    }
  };
}

impl_approx_eq!(f32);
impl_approx_eq!(f64);

/// Allows computing a ping pong value.
pub trait PingPong {
  fn ping_pong(&self) -> Self;
}

macro_rules! impl_ping_pong {
  ($type:ty) => {
    impl PingPong for $type {
      #[inline(always)]
      fn ping_pong(&self) -> Self {
        self.sin() * 2.0 - 1.0
      }
    }
  };
}

impl_ping_pong!(f32);
impl_ping_pong!(f64);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_approx_eq_f32() {
    let value1 = 0.5f32;
    let value2 = 0.500001f32;

    assert!(value1.approx_eq(value2));
  }

  #[test]
  fn test_approx_eq_f64() {
    let value1 = 0.5f64;
    let value2 = 0.5000000001f64;

    assert!(value1.approx_eq(value2));
  }

  #[test]
  fn test_ping_pong_f32() {
    let value = 0.5f32;

    let expected_result = value.sin() * 2.0 - 1.0;

    assert_eq!(value.ping_pong(), expected_result);
  }

  #[test]
  fn test_ping_pong_f64() {
    let value = 0.5f64;

    let expected_result = value.sin() * 2.0 - 1.0;

    assert_eq!(value.ping_pong(), expected_result);
  }
}
