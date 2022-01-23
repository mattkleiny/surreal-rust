use std::fmt::Formatter;

/// Represents a path in a virtual file system.
#[derive(Copy, Clone)]
pub struct VirtualPath<'a> {
  scheme: &'a str,
  location: &'a str,
}

impl<'a> VirtualPath<'a> {
  /// Parses the given string-like object into a path with scheme and location.
  pub fn parse<S: AsRef<str> + ?Sized>(raw: &'a S) -> Self {
    let raw = raw.as_ref();
    let split: Vec<&str> = raw.split("://").collect();

    if split.len() != 2 {
      return Self {
        scheme: "local",
        location: split[0],
      };
    }

    Self {
      scheme: split[0],
      location: split[1],
    }
  }
}

impl<'a> std::fmt::Debug for VirtualPath<'a> {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Ok(write!(formatter, "{:}://{:}", self.scheme, self.location)?)
  }
}

impl<'a> std::fmt::Display for VirtualPath<'a> {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
    Ok(write!(formatter, "{:}://{:}", self.scheme, self.location)?)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn path_should_parse_simple_schemes() {
    let path = VirtualPath::parse("local://README.md");

    assert_eq!("local", path.scheme);
    assert_eq!("README.md", path.location);
    assert_eq!("local://README.md", format!("{:}", path));
  }
}
