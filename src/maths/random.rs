use std::cell::RefCell;

use super::{Lerp, Numeric};

/// A pseudo-random number generator.
#[derive(Clone, Debug)]
pub struct Random {
  state: u64,
}

impl Default for Random {
  fn default() -> Self {
    Self::new()
  }
}

impl Random {
  /// Constructs a random generator with a random seed.
  pub fn new() -> Self {
    Self::with_seed(generate_thread_local())
  }

  /// Constructs a random generator with the given seed.
  pub fn with_seed(seed: u64) -> Self {
    Random { state: seed }
  }

  /// Generates a new value of the given [`Random`] type, T.
  pub fn next<T: FromRandom>(&mut self) -> T {
    T::from_random(self)
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

/// Generates a new f64 using the thread-local generator.
fn generate_thread_local<T>() -> T
where
  T: FromRandom,
{
  THREAD_LOCAL_RANDOM.with(|random| {
    let mut random = random.borrow_mut();

    T::from_random(&mut random)
  })
}

/// A type that can be randomly generated.
pub trait FromRandom: Sized {
  /// Generates a new value of this type with a global random seed.
  fn random() -> Self {
    generate_thread_local()
  }

  /// Generates a new random value of this type using the given generator.
  fn from_random(random: &mut Random) -> Self;
}

// commonly used random types
impl FromRandom for bool {
  fn from_random(generator: &mut Random) -> Self {
    generator.next_f64() < 0.5
  }
}

impl FromRandom for u8 {
  fn from_random(generator: &mut Random) -> Self {
    generator.next_u64() as u8
  }
}

impl FromRandom for u16 {
  fn from_random(generator: &mut Random) -> Self {
    generator.next_u64() as u16
  }
}

impl FromRandom for u32 {
  fn from_random(generator: &mut Random) -> Self {
    generator.next_u64() as u32
  }
}

impl FromRandom for u64 {
  fn from_random(generator: &mut Random) -> Self {
    generator.next_u64()
  }
}

impl FromRandom for f32 {
  fn from_random(generator: &mut Random) -> Self {
    generator.next_f64() as f32
  }
}

impl FromRandom for f64 {
  fn from_random(generator: &mut Random) -> Self {
    generator.next_f64()
  }
}

/// Allows an item to be selected randomly.
pub trait RandomSelect {
  type Item;

  /// Selects an item randomly from the object.
  fn select_randomly(&self, random: &mut Random) -> &Self::Item;
}

/// Allows random selection from a slice of [`T`].
impl<T> RandomSelect for &[T] {
  type Item = T;

  fn select_randomly(&self, random: &mut Random) -> &Self::Item {
    &self[random.next_u64() as usize % self.len()]
  }
}

/// Allows random selection from a [`Vec<T>`].
impl<T> RandomSelect for &Vec<T> {
  type Item = T;

  fn select_randomly(&self, random: &mut Random) -> &Self::Item {
    &self[random.next_u64() as usize % self.len()]
  }
}

/// Distribution modes for `RandomVariable`s.
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

/// A variable of T that allows random sampling.
#[derive(Clone, Debug)]
pub struct RandomVariable<T> {
  pub low: T,
  pub high: T,
  pub distribution: Distribution,
}

impl<T: Numeric> Default for RandomVariable<T> {
  fn default() -> Self {
    Self {
      low: T::ZERO,
      high: T::ONE,
      distribution: Distribution::Uniform,
    }
  }
}

impl<T: Numeric + Lerp> RandomVariable<T> {
  /// Samples the random variable at the given `t`.
  #[inline]
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
  fn random_variable_should_sample_uniformly() {
    let variable = RandomVariable {
      low: 0.,
      high: 1.,
      distribution: Distribution::Uniform,
    };

    assert_eq!(0., variable.sample(0.));
    assert_eq!(1., variable.sample(1.));
  }
}
