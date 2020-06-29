/// Abstracts over the window provider of a device.
///
/// Permits interaction with the underlying window API through a higher-level abstraction.
pub trait WindowManager {
  fn set_title(&mut self, title: impl AsRef<str>) -> Result<(), WindowError>;
}

/// Represents an error with windowing.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WindowError {
  GeneralFailure,
}

impl From<WindowError> for crate::Error {
  fn from(_: WindowError) -> Self {
    crate::Error::Platform
  }
}
