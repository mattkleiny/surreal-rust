use std::fmt::{Display, Formatter};

use serde::{Deserializer, Serializer};

/// A version identifier `major.minor.patch`.
///
/// Used to indicate versions of projects, assets, etc.
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Version {
  major: u16,
  minor: u16,
  patch: u16,
}

impl Version {
  /// Creates a new [`Version`].
  #[inline(always)]
  pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
    Self {
      major,
      minor,
      patch,
    }
  }

  /// Parses a [`Version`] from the given string.
  pub fn parse(string: &str) -> crate::Result<Self> {
    let mut parts = string.split('.');

    let major = parts
      .next()
      .ok_or(crate::anyhow!("Missing major component"))?
      .parse()?;

    let minor = parts
      .next()
      .ok_or(crate::anyhow!("Missing minor component"))?
      .parse()?;

    let patch = parts
      .next()
      .ok_or(crate::anyhow!("Missing patch component"))?
      .parse()?;

    Ok(Self {
      major,
      minor,
      patch,
    })
  }
}

impl Display for Version {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
  }
}

impl serde::Serialize for Version {
  fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&format!("{}.{}.{}", self.major, self.minor, self.patch))
  }
}

impl<'de> serde::Deserialize<'de> for Version {
  fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    struct Visitor;

    impl<'de> serde::de::Visitor<'de> for Visitor {
      type Value = Version;

      fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a version string")
      }

      fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
        let mut parts = value.split('.');

        let major = parts.next().unwrap().parse::<u16>().unwrap();
        let minor = parts.next().unwrap().parse::<u16>().unwrap();
        let patch = parts.next().unwrap().parse::<u16>().unwrap();

        Ok(Self::Value {
          major,
          minor,
          patch,
        })
      }
    }

    deserializer.deserialize_str(Visitor)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::io::{Deserializable, Serializable};

  #[test]
  fn version_should_serialize_and_deserialize() {
    let version1 = Version::new(1, 2, 3);
    let version2 = Version::from_json(&version1.to_json().unwrap()).unwrap();

    assert_eq!(version1, version2);
  }
}
