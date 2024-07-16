use std::fmt::{Display, Formatter};

/// An error that occurs when parsing a [`Version`] from a string.
#[derive(Debug)]
pub enum VersionParseError {
  MissingMajor,
  InvalidMajor,
  MissingMinor,
  InvalidMinor,
  MissingPatch,
  InvalidPatch,
}

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
    Self { major, minor, patch }
  }

  /// Parses a [`Version`] from the given string.
  pub fn parse(string: &str) -> Result<Self, VersionParseError> {
    let mut parts = string.split('.');

    let major = parts
      .next()
      .ok_or(VersionParseError::MissingMajor)?
      .parse()
      .map_err(|_| VersionParseError::InvalidMajor)?;

    let minor = parts
      .next()
      .ok_or(VersionParseError::MissingMinor)?
      .parse()
      .map_err(|_| VersionParseError::InvalidMinor)?;

    let patch = parts
      .next()
      .ok_or(VersionParseError::MissingPatch)?
      .parse()
      .map_err(|_| VersionParseError::InvalidPatch)?;

    Ok(Self { major, minor, patch })
  }
}

impl Display for Version {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new() {
    let version = Version::new(1, 2, 3);

    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 2);
    assert_eq!(version.patch, 3);
  }

  #[test]
  fn test_parse_valid() {
    let version = Version::parse("1.2.3").unwrap();

    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 2);
    assert_eq!(version.patch, 3);
  }
}
