/// A seed for random generation.
///
/// Seeds can be passed around efficiently and turned into an `RNG` easily.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct Seed(u64);

impl Seed {
  /// Creates a new seed using the given value.
  pub const fn new(seed: u64) -> Self {
    Self(seed)
  }

  /// Generates a new seed using a new random value.
  pub fn random() -> Self {
    Self::new(unimplemented!())
  }

  /// Converts the seed into an `RNG`.
  pub fn to_rng(&self) -> RandomGenerator {
    if self.0 == 0 {
      RandomGenerator::with_random_seed()
    } else {
      RandomGenerator::with_seed(self.0)
    }
  }
}

/// A type that can be randomly generated.
pub trait Random: Sized {
  /// Generates a new value of this type with a global random seed.
  fn random() -> Self {
    Self::generate(&mut Seed::random().to_rng())
  }

  /// Generates a new random value of this type using the given generator.
  fn generate(generator: &mut RandomGenerator) -> Self;
}

impl Random for u8 {
  fn generate(generator: &mut RandomGenerator) -> Self {
    unimplemented!()
  }
}

impl Random for u16 {
  fn generate(generator: &mut RandomGenerator) -> Self {
    unimplemented!()
  }
}

impl Random for u32 {
  fn generate(generator: &mut RandomGenerator) -> Self {
    unimplemented!()
  }
}

impl Random for u64 {
  fn generate(generator: &mut RandomGenerator) -> Self {
    unimplemented!()
  }
}

impl Random for Seed {
  fn generate(generator: &mut RandomGenerator) -> Self {
    Self(generator.next())
  }
}

/// A pseudo-random number generator.
#[derive(Clone, Debug)]
pub struct RandomGenerator {
  state: u64,
}

impl RandomGenerator {
  pub fn with_seed(seed: u64) -> Self {
    Self { state: seed }
  }

  pub fn with_random_seed() -> Self {
    Self::with_seed(unimplemented!())
  }

  #[inline(always)]
  pub fn next<T>(&mut self) -> T where T: Random {
    T::generate(self)
  }

  #[inline]
  pub fn next_u64(&mut self) -> u64 {
    unimplemented!()
  }
}

impl Default for RandomGenerator {
  fn default() -> Self {
    RandomGenerator::with_random_seed()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn seed_should_generate_a_valid_rng() {
    let mut rng = Seed::random().to_rng();

    let first: f64 = rng.next();
    let second: f64 = rng.next();

    assert_ne!(first, second);
  }
}
