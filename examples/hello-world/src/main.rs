use surreal::platform::desktop::*;
use surreal::prelude::*;

fn main() -> std::result::Result<(), SurrealError> {
  let platform = DesktopPlatform::new(Configuration {
    title: "Hello, World!",
    size: (1920, 1080),
  })?;

  platform.run(|platform| {
    platform.graphics().clear_active_frame_buffer(Color::PINK);
  });

  Ok(())
}
