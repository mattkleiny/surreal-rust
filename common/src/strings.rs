//! Helpers for working with strings.

pub use names::*;

mod names;

/// Implements owned and borrowed string conversions for a type.
#[macro_export]
macro_rules! impl_cow_string {
  ($type:ident) => {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct $type<'a>(std::borrow::Cow<'a, str>);

    impl<'a> From<&'a str> for $type<'a> {
      fn from(value: &'a str) -> Self {
        Self(std::borrow::Cow::Borrowed(value))
      }
    }

    impl<'a> From<String> for $type<'a> {
      fn from(value: String) -> Self {
        Self(std::borrow::Cow::Owned(value))
      }
    }
  };
}
