use crate::platform::Window;

use super::DesktopPlatform;

impl Window for DesktopPlatform {
  fn set_title(&mut self, title: impl AsRef<str>) {
    self.window_context.window().set_title(title.as_ref());
  }
}
