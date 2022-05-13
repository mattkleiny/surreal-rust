/// A type that can be randomly generated.
pub trait FromRandom: Sized {
  /// Generates a new value of this type with a global random seed.
  fn random() -> Self {
    Self::from_random(&mut Seed::generate().to_random())
  }

  /// Generates a new random value of this type using the given generator.
  fn from_random(random: &mut Random) -> Self;
}

/// A pseudo-random number generator.
#[derive(Clone, Debug)]
pub struct Random {
  next: i32,
  next_prime: i32,
  seed_array: [i32; 56],
}

impl Random {
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
  pub fn next<T>(&mut self) -> T where T: FromRandom {
    T::from_random(self)
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

impl Default for Random {
  fn default() -> Self {
    Random::with_random_seed()
  }
}

/// A seed for random generation.
///
/// Seeds can be passed around efficiently and turned into a random generator easily.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct Seed(u64);

impl Seed {
  /// Creates a new seed using the given value.
  pub const fn new(seed: u64) -> Self {
    Self(seed)
  }

  /// Generates a new seed using a new random value.
  pub fn generate() -> Self {
    todo!()
  }

  /// Converts the seed into a `Random` generator.
  pub fn to_random(&self) -> Random {
    if self.0 == 0 {
      Random::with_random_seed()
    } else {
      Random::with_seed(self.0)
    }
  }
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

impl FromRandom for Seed {
  fn from_random(generator: &mut Random) -> Self {
    Self(generator.next())
  }
}