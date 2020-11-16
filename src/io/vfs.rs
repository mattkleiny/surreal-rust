use super::{Error, IOResult};

/// Represents a path in a virtual file system.
#[derive(Copy, Clone)]
pub struct Path<'a> {
  scheme: &'a str,
  location: &'a str,
}

impl<'a> Path<'a> {
  /// Parses the given string-like object into a path with scheme and location.
  pub fn parse<S: AsRef<str> + ?Sized>(raw: &'a S) -> IOResult<Self> {
    let raw = raw.as_ref();
    let split: Vec<&str> = raw.split("://").collect();

    if split.len() != 2 {
      return Err(Error::InvalidPathScheme);
    }

    Ok(Self {
      scheme: split[0],
      location: split[1],
    })
  }
}

impl<'a> std::fmt::Debug for Path<'a> {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Ok(write!(formatter, "{:?}://{:?}", self.scheme, self.location)?)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_parse_simple_schemes() {
    let path = Path::parse("local://README.md").unwrap();

    assert_eq!("local", path.scheme);
    assert_eq!("README.md", path.location);
  }
}
