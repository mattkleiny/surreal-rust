//! Abstractions over the windowing service for the platform.

pub trait WindowServer {
  fn set_title(&mut self, title: impl AsRef<str>);
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WindowError {
  GeneralFailure,
}

impl From<WindowError> for crate::Error {
  fn from(_: WindowError) -> Self {
    crate::Error::Platform
  }
}
