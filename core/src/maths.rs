//! Mathematical utilities.

pub use angles::*;
pub use curves::*;
pub use hex::*;
pub use interpolation::*;
pub use linear::*;
pub use neighbours::*;
pub use numbers::*;
pub use paths::*;
pub use random::*;
pub use ranges::*;
pub use rectangles::*;
pub use shapes::*;

mod angles;
mod curves;
mod hex;
mod interpolation;
mod linear;
mod neighbours;
mod numbers;
mod paths;
mod random;
mod ranges;
mod rectangles;
mod shapes;

/// A globally unique identifier.
pub type Guid = uuid::Uuid;

/// Allows approximate equality checks between values.
pub trait ApproxEq<T = Self> {
  fn approx_eq(&self, other: T) -> bool;
}

macro_rules! impl_approx_eq {
  ($type:ty) => {
    impl ApproxEq for $type {
      #[inline]
      fn approx_eq(&self, other: Self) -> bool {
        const EPSILON: $type = 0.00001;

        (other - self).abs() < EPSILON
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
      #[inline]
      fn ping_pong(&self) -> Self {
        self.sin() * 2.0 - 1.0
      }
    }
  };
}

impl_ping_pong!(f32);
impl_ping_pong!(f64);
