use rand::prelude::*;

/// A seed for random generation.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct Seed(u64);

impl Seed {
  /// Generates a new seed using a new random value.
  pub fn random() -> Self {
    Self(random_u64())
  }

  /// Converts the seed into an `RandomGenerator`.
  #[inline]
  pub fn to_random(&self) -> RandomGenerator {
    if self.0 == 0 {
      RandomGenerator::new(random_u64())
    } else {
      RandomGenerator::new(self.0)
    }
  }
}

/// A type that can be randomly generated.
pub trait Random: Sized {
  /// Generates a new value of this type with a global random seed.
  fn random_global() -> Self { Self::random(&mut Seed::random().to_random()) }

  /// Generates a new random value of this type using the given generator.
  fn random(generator: &mut RandomGenerator) -> Self;
}

/// Adapt all standard distribution types to our custom Random interface.
impl<T> Random for T where rand::distributions::Standard: Distribution<T> {
  fn random(generator: &mut RandomGenerator) -> Self {
    generator.next()
  }
}

/// A pseudo-random number generator.
#[derive(Clone, Debug)]
pub struct RandomGenerator {
  rng: StdRng,
}

impl RandomGenerator {
  pub fn new(seed: u64) -> Self {
    Self { rng: StdRng::seed_from_u64(seed) }
  }

  #[inline]
  pub fn next<T: Random>(&mut self) -> T {
    T::random(self)
  }
}

impl Default for RandomGenerator {
  fn default() -> Self {
    RandomGenerator::new(random_u64())
  }
}

#[inline]
fn random_u64() -> u64 {
  rand::thread_rng().next_u64()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn seed_should_generate_a_valid_rng() {
    let seed = Seed::random();
    let mut rng = seed.to_random();

    let first: f64 = rng.next();
    let second: f64 = rng.next();

    assert_ne!(first, second);
  }
}
