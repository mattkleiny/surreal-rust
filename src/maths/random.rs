//! Random number generation

use rand::prelude::*;

/// A seed for random generation.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Seed(u64);

impl Seed {
  /// Generates a new seed using a new random value.
  pub fn random() -> Self {
    Self(rand::thread_rng().gen())
  }

  /// Converts the seed into an RNG.
  #[inline]
  pub fn to_rng(&self) -> RNG {
    RNG::new(self.0)
  }
}

/// A random number generator.
#[derive(Clone, Debug)]
pub struct RNG {
  rng: StdRng,
}

impl RNG {
  pub fn new(seed: u64) -> Self {
    Self {
      rng: StdRng::seed_from_u64(seed)
    }
  }

  #[inline]
  pub fn next_u32(&mut self) -> u32 { self.rng.gen() }

  #[inline]
  pub fn next_u64(&mut self) -> u64 { self.rng.gen() }

  #[inline]
  pub fn next_f32(&mut self) -> f32 { self.rng.gen() }

  #[inline]
  pub fn next_f64(&mut self) -> f64 { self.rng.gen() }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn seed_should_generate_a_valid_rng() {
    let seed = Seed::random();
    let mut rng = seed.to_rng();

    let first = rng.next_f64();
    let second = rng.next_f64();

    assert_ne!(first, second);
  }
}
