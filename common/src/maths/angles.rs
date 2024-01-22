use std::fmt::Display;

use super::{FromRandom, Random};

/// Represents an angle in either degrees or radians.
#[repr(C)]
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub enum Angle {
  Radians(f64),
  Degrees(f64),
}

impl Angle {
  pub const ZERO: Self = Self::Radians(0.0);
}

impl Default for Angle {
  #[inline(always)]
  fn default() -> Self {
    Self::Radians(0.0)
  }
}

impl FromRandom for Angle {
  fn from_random(random: &mut Random) -> Self {
    Self::Radians(random.next_f64() * std::f64::consts::PI * 2.0)
  }
}

impl Display for Angle {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Angle::Radians(radians) => write!(formatter, "{}rad", radians),
      Angle::Degrees(degress) => write!(formatter, "{}°", degress),
    }
  }
}

impl From<Angle> for f32 {
  fn from(value: Angle) -> Self {
    match value {
      Angle::Radians(radians) => radians as f32,
      Angle::Degrees(degrees) => degrees.to_radians() as f32,
    }
  }
}
impl From<Angle> for f64 {
  fn from(value: Angle) -> Self {
    match value {
      Angle::Radians(radians) => radians,
      Angle::Degrees(degrees) => degrees.to_radians(),
    }
  }
}
