/// A type that can be randomly generated.
pub trait Random: Sized {
  /// Generates a new value of this type with a global random seed.
  fn random() -> Self {
    Self::generate(&mut Seed::random().to_rng())
  }

  /// Generates a new random value of this type using the given generator.
  fn generate(generator: &mut RandomGenerator) -> Self;
}

/// A pseudo-random number generator.
#[derive(Clone, Debug)]
pub struct RandomGenerator {
  next: i32,
  next_prime: i32,
  seed_array: [i32; 56]
}

impl RandomGenerator {
  const SEED_CONSTANT: u32 = 161803398;

  /// Constructs a random generator with the given seed.
  pub fn with_seed(seed: u64) -> Self {
    todo!()
  }

  /// Constructs a random generator with a random seed.
  pub fn with_random_seed() -> Self {
    todo!()
  }

  /// Generates a new value of the given `Random` type, T.
  #[inline(always)]
  pub fn next<T>(&mut self) -> T where T: Random {
    T::generate(self)
  }

  /// Generates a random u64 number between 0 and u64::MAX, inclusive.
  pub fn next_u64(&mut self) -> u64 {
    unimplemented!()
  }

  /// Generates a random normalized f64 number between 0. and 1., inclusive.
  pub fn next_f64(&mut self) -> f64 {
    unimplemented!()
  }
}

impl Default for RandomGenerator {
  fn default() -> Self {
    RandomGenerator::with_random_seed()
  }
}

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
    todo!()
  }

  /// Converts the seed into an `Rng`.
  pub fn to_rng(&self) -> RandomGenerator {
    if self.0 == 0 {
      RandomGenerator::with_random_seed()
    } else {
      RandomGenerator::with_seed(self.0)
    }
  }
}

// commonly used random types

impl Random for bool {
  fn generate(generator: &mut RandomGenerator) -> Self {
    generator.next_f64() < 0.5
  }
}

impl Random for u8 {
  fn generate(generator: &mut RandomGenerator) -> Self {
    generator.next_u64() as u8
  }
}

impl Random for u16 {
  fn generate(generator: &mut RandomGenerator) -> Self {
    generator.next_u64() as u16
  }
}

impl Random for u32 {
  fn generate(generator: &mut RandomGenerator) -> Self {
    generator.next_u64() as u32
  }
}

impl Random for u64 {
  fn generate(generator: &mut RandomGenerator) -> Self {
    generator.next_u64()
  }
}

impl Random for f32 {
  fn generate(generator: &mut RandomGenerator) -> Self {
    generator.next_f64() as f32
  }
}

impl Random for f64 {
  fn generate(generator: &mut RandomGenerator) -> Self {
    generator.next_f64()
  }
}

impl Random for Seed {
  fn generate(generator: &mut RandomGenerator) -> Self {
    Self(generator.next())
  }
}