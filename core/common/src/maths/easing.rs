use crate::Scalar;

/// A tweening function.
pub type Easing<T> = fn(T, T, f32) -> T;

/// A trait for types that can be tweened.
pub trait Tweenable: Sized {
  /// Tween the value towards the target.
  fn tween(a: Self, b: Self, delta: f32, easing: Easing<Self>) -> Self {
    easing(a, b, delta)
  }
}

/// Linear easing.
pub fn linear<T: Scalar>(a: T, b: T, t: f32) -> T {
  let a = a.to_f32();
  let b = b.to_f32();

  T::from_f32(a + (b - a) * t)
}

/// Quadratic in easing.
pub fn quadratic_in<T: Scalar>(a: T, b: T, t: f32) -> T {
  let a = a.to_f32();
  let b = b.to_f32();

  T::from_f32(a + (b - a) * t * t)
}

/// Quadratic out easing.
pub fn quadratic_out<T: Scalar>(a: T, b: T, t: f32) -> T {
  let a = a.to_f32();
  let b = b.to_f32();

  T::from_f32(a + (b - a) * (1.0 - t) * (1.0 - t))
}

/// Quadratic in-out easing.
pub fn quadratic_in_out<T: Scalar>(a: T, b: T, t: f32) -> T {
  if t < 0.5 {
    quadratic_in(a, b, t * 2.0)
  } else {
    quadratic_out(a, b, t * 2.0 - 1.0)
  }
}

/// Cubic in easing.
pub fn cubic_in<T: Scalar>(a: T, b: T, t: f32) -> T {
  let a = a.to_f32();
  let b = b.to_f32();

  T::from_f32(a + (b - a) * t * t * t)
}

/// Cubic out easing.
pub fn cubic_out<T: Scalar>(a: T, b: T, t: f32) -> T {
  let a = a.to_f32();
  let b = b.to_f32();
  let t = t - 1.0;

  T::from_f32(a + (b - a) * (t * t * t + 1.0))
}

/// Cubic in-out easing.
pub fn cubic_in_out<T: Scalar>(a: T, b: T, t: f32) -> T {
  if t < 0.5 {
    cubic_in(a, b, t * 2.0)
  } else {
    cubic_out(a, b, t * 2.0 - 1.0)
  }
}

/// Quartic in easing.
pub fn quartic_in<T: Scalar>(a: T, b: T, t: f32) -> T {
  let a = a.to_f32();
  let b = b.to_f32();

  T::from_f32(a + (b - a) * t * t * t * t)
}

/// Quartic out easing.
pub fn quartic_out<T: Scalar>(a: T, b: T, t: f32) -> T {
  let a = a.to_f32();
  let b = b.to_f32();
  let t = t - 1.0;

  T::from_f32(a + (b - a) * (1.0 - t * t * t * t))
}

/// Quartic in-out easing.
pub fn quartic_in_out<T: Scalar>(a: T, b: T, t: f32) -> T {
  if t < 0.5 {
    quartic_in(a, b, t * 2.0)
  } else {
    quartic_out(a, b, t * 2.0 - 1.0)
  }
}
