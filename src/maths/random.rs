use rand::prelude::*;

/// A seed for random generation.
///
/// Seeds can be passed around efficiently and turned into `RandomGenerator` easily.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct Seed(u64);

impl Seed {
  /// Generates a new seed using a new random value.
  pub fn random() -> Self {
    Self(random_u64())
  }

  /// Converts the seed into an `RandomGenerator`.
  pub fn to_random(&self) -> RNG {
    if self.0 == 0 {
      RNG::new(random_u64())
    } else {
      RNG::new(self.0)
    }
  }
}

/// A type that can be randomly generated.
pub trait Random: Sized {
  /// Generates a new value of this type with a global random seed.
  fn random_global() -> Self {
    Self::random(&mut Seed::random().to_random())
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
  pub fn new(seed: u64) -> Self {
    Self {
      rng: StdRng::seed_from_u64(seed),
    }
  }

  pub fn next<T: Random>(&mut self) -> T {
    T::random(self)
  }
}

impl Default for RNG {
  fn default() -> Self {
    RNG::new(random_u64())
  }
}

fn random_u64() -> u64 {
  rand::thread_rng().next_u64()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn seed_should_generate_a_valid_rng() {
    let mut rng = Seed::random().to_random();

    let first: f64 = rng.next();
    let second: f64 = rng.next();

    assert_ne!(first, second);
  }
}
