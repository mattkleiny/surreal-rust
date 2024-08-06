use std::ops::{Deref, DerefMut};

/// Represents either an owned or borrowed type [`T`].
///
/// This is useful for when you want to have a type that can be either owned or
/// borrowed, but you don't want to use [`Cow`] because it's too heavy.
pub enum MaybeOwned<'a, T: ?Sized> {
  Owned(Box<T>),
  Borrowed(&'a mut T),
}

impl<'a, T: ?Sized> MaybeOwned<'a, T> {
  /// Gets a reference to the inner value.
  #[inline]
  pub fn as_ref(&self) -> &T {
    match self {
      Self::Owned(stream) => stream.as_ref(),
      Self::Borrowed(stream) => *stream,
    }
  }

  /// Gets a mutable reference to the inner value.
  #[inline]
  pub fn as_mut(&mut self) -> &mut T {
    match self {
      Self::Owned(stream) => stream.as_mut(),
      Self::Borrowed(stream) => *stream,
    }
  }
}

impl<'a, T: ?Sized> From<&'a mut T> for MaybeOwned<'a, T> {
  #[inline]
  fn from(value: &'a mut T) -> Self {
    Self::Borrowed(value)
  }
}

impl<T> From<T> for MaybeOwned<'static, T> {
  #[inline]
  fn from(value: T) -> Self {
    Self::Owned(Box::new(value))
  }
}

impl<'a, T: ?Sized> Deref for MaybeOwned<'a, T> {
  type Target = T;

  #[inline]
  fn deref(&self) -> &Self::Target {
    self.as_ref()
  }
}

impl<'a, T: ?Sized> DerefMut for MaybeOwned<'a, T> {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.as_mut()
  }
}
