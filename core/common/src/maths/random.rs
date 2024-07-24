use std::cell::RefCell;

use uuid::Uuid;

use super::{Lerp, Scalar};

/// A pseudo-random number generator.
#[derive(Clone, Debug)]
pub struct Random {
  state: u64,
}

impl Default for Random {
  #[inline]
  fn default() -> Self {
    Self::with_thread_local_seed()
  }
}

impl Random {
  /// Constructs a random generator with the given seed.
  #[inline]
  pub fn with_seed(seed: u64) -> Self {
    Random { state: seed }
  }

  /// Constructs a random generator with a random seed.
  #[inline]
  pub fn with_thread_local_seed() -> Self {
    Self::with_seed(u64::random())
  }

  /// Generates a new value of the given [`Random`] type, T.
  #[allow(clippy::should_implement_trait)]
  pub fn next<T: FromRandom>(&mut self) -> T {
    T::from_random(self)
  }

  /// Generates a random [`Scalar`] between the given range.
  pub fn next_range<T: FromRandom + Scalar>(&mut self, range: std::ops::Range<T>) -> T {
    range.start + (self.next::<T>() % (range.end - range.start))
  }

  /// Generates a random u64 number between 0 and u64::MAX, inclusive.
  pub fn next_u64(&mut self) -> u64 {
    self.state = self.state.wrapping_add(0xA0761D6478BD642F);
    let value = u128::from(self.state) * u128::from(self.state ^ 0xE7037ED1A0B428DB);

    (value as u64) ^ (value >> 64) as u64
  }

  /// Generates a random normalized f64 number between 0. and 1., inclusive.
  pub fn next_f64(&mut self) -> f64 {
    let b = 64;
    let f = f64::MANTISSA_DIGITS - 1;

    f64::from_bits((1 << (b - 2)) - (1 << f) + (self.next_u64() >> (b - f))) - 1.0
  }
}

thread_local! {
  /// A thread-local instance of the [`Random`].
  static THREAD_LOCAL_RANDOM: RefCell<Random> = RefCell::new(Random::with_seed({
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    // build a hash of the current time and the current thread id and use this as the seed
    let mut hasher = DefaultHasher::new();

    std::time::Instant::now().hash(&mut hasher);
    std::thread::current().id().hash(&mut hasher);

    (hasher.finish() << 1) | 1
  }));
}

/// A type that can be randomly generated.
pub trait FromRandom: Sized {
  /// Generates a new value of this type with a global random number generator.
  fn random() -> Self {
    THREAD_LOCAL_RANDOM.with(|random| {
      let mut random = random.borrow_mut();

      Self::from_random(&mut random)
    })
  }

  /// Generates a new random value of this type using the given generator.
  fn from_random(random: &mut Random) -> Self;
}

// Implements random conversion for common integer types
macro_rules! impl_random_number {
  ($type:ty) => {
    impl FromRandom for $type {
      #[inline]
      fn from_random(random: &mut Random) -> Self {
        random.next_u64() as $type
      }
    }
  };
}

impl_random_number!(u8);
impl_random_number!(u16);
impl_random_number!(u32);
impl_random_number!(u64);
impl_random_number!(usize);

impl_random_number!(i8);
impl_random_number!(i16);
impl_random_number!(i32);
impl_random_number!(i64);
impl_random_number!(isize);

impl FromRandom for bool {
  #[inline]
  fn from_random(random: &mut Random) -> Self {
    random.next_f64() < 0.5
  }
}

impl FromRandom for f32 {
  #[inline]
  fn from_random(random: &mut Random) -> Self {
    random.next_f64() as f32
  }
}

impl FromRandom for f64 {
  #[inline]
  fn from_random(random: &mut Random) -> Self {
    random.next_f64()
  }
}

impl FromRandom for Uuid {
  #[inline]
  fn from_random(random: &mut Random) -> Self {
    Uuid::from_bytes(random.next())
  }
}

/// Generates fixed-length arrays of `T` where `T` itself is [`FromRandom`]
impl<T, const L: usize> FromRandom for [T; L]
where
  T: FromRandom + Default + Copy,
{
  fn from_random(random: &mut Random) -> Self {
    let mut result = [T::default(); L];

    for element in &mut result {
      *element = T::from_random(random);
    }

    result
  }
}

/// Distribution modes for [`RandomVariable`]s.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Distribution {
  Uniform,
  Square,
  Cube,
  Fourth,
  OneMinusSquare,
  OneMinusCube,
  OneMinusFourth,
}

/// A variable of [`T`] that allows random sampling.
#[derive(Clone, Debug)]
pub struct RandomVariable<T> {
  pub low: T,
  pub high: T,
  pub distribution: Distribution,
}

impl<T: Scalar> Default for RandomVariable<T> {
  #[inline]
  fn default() -> Self {
    Self {
      low: T::ZERO,
      high: T::ONE,
      distribution: Distribution::Uniform,
    }
  }
}

impl<T: Copy + Lerp> RandomVariable<T> {
  // Samples a value from the variable at the given t.
  pub fn sample(&self, t: f32) -> T {
    use Distribution::*;

    let x = match self.distribution {
      Uniform => t,
      Square => t * t,
      Cube => t * t * t,
      Fourth => t * t * t * t,
      OneMinusSquare => 1. - t * t,
      OneMinusCube => 1. - t * t * t,
      OneMinusFourth => 1. - t * t * t * t,
    };

    T::lerp(self.low, self.high, x)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_generate_different_values() {
    let mut random = Random::with_seed(0);

    let a = random.next_u64();
    let b = random.next_u64();

    assert_ne!(a, b);
  }

  #[test]
  fn test_generate_different_values_with_different_seeds() {
    let mut random_a = Random::with_seed(0);
    let mut random_b = Random::with_seed(1);

    let a = random_a.next_u64();
    let b = random_b.next_u64();

    assert_ne!(a, b);
  }

  #[test]
  fn test_generate_same_values_with_same_seeds() {
    let mut random_a = Random::with_seed(0);
    let mut random_b = Random::with_seed(0);

    let a = random_a.next_u64();
    let b = random_b.next_u64();

    assert_eq!(a, b);
  }

  #[test]
  fn test_generate_value_based_on_global_random() {
    let a = u64::random();
    let b = u64::random();

    assert_ne!(a, b);
  }

  macro_rules! impl_variable_test {
    ($name:ident; $distribution:ident => $low:literal, $mid:literal, $high:literal) => {
      #[test]
      fn $name() {
        let variable: RandomVariable<f32> = RandomVariable {
          distribution: Distribution::$distribution,
          ..Default::default()
        };

        assert_eq!($low, variable.sample(0.));
        assert_eq!($mid, variable.sample(0.5));
        assert_eq!($high, variable.sample(1.));
      }
    };
  }

  impl_variable_test!(random_variable_should_sample_uniformly; Uniform => 0.0, 0.5, 1.0);
  impl_variable_test!(random_variable_should_sample_square; Square => 0.0, 0.25, 1.0);
  impl_variable_test!(random_variable_should_sample_cube; Cube => 0.0, 0.125, 1.0);
  impl_variable_test!(random_variable_should_sample_fourth; Fourth => 0.0, 0.0625, 1.0);
  impl_variable_test!(random_variable_should_sample_one_minus_square; OneMinusSquare => 1.0, 0.75, 0.0);
  impl_variable_test!(random_variable_should_sample_one_minus_cube; OneMinusCube => 1.0, 0.875, 0.0);
  impl_variable_test!(random_variable_should_sample_one_minus_fourth; OneMinusFourth => 1.0, 0.9375, 0.0);
}
