use rand::prelude::*;

/// A type that can be randomly generated.
pub trait RNG {
  /// Generates a new random value of this type.
  fn random(random: &mut Random) -> Self;
}

/// A seed for random generation.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct Seed(u64);

impl Seed {
  /// Generates a new seed using a new random value.
  pub fn random() -> Self {
    Self(random_u64())
  }

  /// Converts the seed into an RNG.
  #[inline]
  pub fn to_random(&self) -> Random {
    if self.0 == 0 {
      Random::new(random_u64())
    } else {
      Random::new(self.0)
    }
  }
}

/// A random number generator.
#[derive(Clone, Debug)]
pub struct Random {
  rng: StdRng,
}

impl Random {
  pub fn new(seed: u64) -> Self {
    Self { rng: StdRng::seed_from_u64(seed) }
  }

  #[inline] pub fn next_u8(&mut self)  -> u8 { self.rng.gen() }
  #[inline] pub fn next_u16(&mut self) -> u16 { self.rng.gen() }
  #[inline] pub fn next_u32(&mut self) -> u32 { self.rng.gen() }
  #[inline] pub fn next_u64(&mut self) -> u64 { self.rng.gen() }
  #[inline] pub fn next_i8(&mut self)  -> i8 { self.rng.gen() }
  #[inline] pub fn next_i16(&mut self) -> i16 { self.rng.gen() }
  #[inline] pub fn next_i32(&mut self) -> i32 { self.rng.gen() }
  #[inline] pub fn next_i64(&mut self) -> i64 { self.rng.gen() }
  #[inline] pub fn next_f32(&mut self) -> f32 { self.rng.gen() }
  #[inline] pub fn next_f64(&mut self) -> f64 { self.rng.gen() }
  #[inline] pub fn next<T: RNG>(&mut self) -> T { T::random(self) }
}

impl Default for Random {
  fn default() -> Self {
    Random::new(random_u64())
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

    let first = rng.next_f64();
    let second = rng.next_f64();

    assert_ne!(first, second);
  }
}
