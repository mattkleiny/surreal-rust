use std::borrow::Cow;

use surreal::collections::FastHashSet;

/// The ID of the layer that a [`SceneNode`] inhabits.
pub type LayerId = u16;

/// A set of one or more [`Tag`]s.
pub type TagSet<'a> = FastHashSet<Tag<'a>>;

/// A tag that can be applied to a [`SceneNode`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tag<'a>(Cow<'a, str>);

impl<'a> From<&'a str> for Tag<'a> {
  fn from(value: &'a str) -> Self {
    Self(Cow::Borrowed(value))
  }
}

impl<'a> From<String> for Tag<'a> {
  fn from(value: String) -> Self {
    Self(Cow::Owned(value))
  }
}
