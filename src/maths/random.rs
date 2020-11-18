use rand::prelude::*;

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
    Self::new(rand::thread_rng().next_u64())
  }

  /// Converts the seed into an `RNG`.
  pub fn to_rng(&self) -> RNG {
    if self.0 == 0 {
      RNG::with_random_seed()
    } else {
      RNG::with_seed(self.0)
    }
  }
}

/// A type that can be randomly generated.
pub trait Random: Sized {
  /// Generates a new value of this type with a global random seed.
  fn random_global() -> Self {
    Self::random(&mut Seed::random().to_rng())
  }

  /// Generates a new random value of this type using the given generator.
  fn random(generator: &mut RNG) -> Self;
}

impl Random for Seed {
  fn random(generator: &mut RNG) -> Self {
    Self(generator.next())
  }
}

/// Adapt all standard distribution types to our custom Random interface.
impl<T> Random for T where rand::distributions::Standard: Distribution<T> {
  fn random(generator: &mut RNG) -> Self {
    generator.rng.sample(rand::distributions::Standard)
  }
}

/// A pseudo-random number generator.
#[derive(Clone, Debug)]
pub struct RNG {
  rng: StdRng,
}

impl RNG {
  pub fn with_seed(seed: u64) -> Self {
    Self { rng: StdRng::seed_from_u64(seed) }
  }

  pub fn with_random_seed() -> Self {
    Self::with_seed(rand::thread_rng().next_u64())
  }

  #[inline(always)]
  pub fn next<T: Random>(&mut self) -> T {
    T::random(self)
  }
}

impl Default for RNG {
  fn default() -> Self {
    RNG::with_random_seed()
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
