use crate::platform::PlatformWindow;

use super::DesktopPlatform;

impl PlatformWindow for DesktopPlatform {
  fn set_title(&mut self, title: impl AsRef<str>) {
    self.window_context.window().set_title(title.as_ref());
  }
}
