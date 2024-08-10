use crate::Lerp;

/// Ane asing function.
pub type Easing<T> = fn(T, T, f32) -> T;

/// Linear easing.
#[inline]
pub fn easing_linear<T: Lerp>(a: T, b: T, t: f32) -> T {
  T::lerp(a, b, t)
}

/// Quadratic in easing.
#[inline]
pub fn easing_quadratic_in<T: Lerp>(a: T, b: T, t: f32) -> T {
  T::lerp(a, b, t * t)
}

/// Quadratic out easing.
#[inline]
pub fn easing_quadratic_out<T: Lerp>(a: T, b: T, t: f32) -> T {
  T::lerp(a, b, 1.0 - t * t)
}

/// Quadratic in-out easing.
#[inline]
pub fn easing_quadratic_in_out<T: Lerp>(a: T, b: T, t: f32) -> T {
  if t < 0.5 {
    easing_quadratic_in(a, b, t * 2.0)
  } else {
    easing_quadratic_out(a, b, t * 2.0 - 1.0)
  }
}

/// Cubic in easing.
#[inline]
pub fn easing_cubic_in<T: Lerp>(a: T, b: T, t: f32) -> T {
  T::lerp(a, b, t * t * t)
}

/// Cubic out easing.
#[inline]
pub fn easing_cubic_out<T: Lerp>(a: T, b: T, t: f32) -> T {
  T::lerp(a, b, 1.0 - t * t * t)
}

/// Cubic in-out easing.
#[inline]
pub fn easing_cubic_in_out<T: Lerp>(a: T, b: T, t: f32) -> T {
  if t < 0.5 {
    easing_cubic_in(a, b, t * 2.0)
  } else {
    easing_cubic_out(a, b, t * 2.0 - 1.0)
  }
}

/// Quartic in easing.
#[inline]
pub fn easing_quartic_in<T: Lerp>(a: T, b: T, t: f32) -> T {
  T::lerp(a, b, t * t * t * t)
}

/// Quartic out easing.
#[inline]
pub fn easing_quartic_out<T: Lerp>(a: T, b: T, t: f32) -> T {
  T::lerp(a, b, 1.0 - t * t * t * t)
}

/// Quartic in-out easing.
#[inline]
pub fn easing_quartic_in_out<T: Lerp>(a: T, b: T, t: f32) -> T {
  if t < 0.5 {
    easing_quartic_in(a, b, t * 2.0)
  } else {
    easing_quartic_out(a, b, t * 2.0 - 1.0)
  }
}
