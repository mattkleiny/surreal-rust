use surreal::platform::desktop::*;
use surreal::prelude::*;

fn main() -> std::result::Result<(), SurrealError> {
  let platform = DesktopPlatform::new(Configuration {
    title: "Hello, World!",
    size: (1920, 1080),
  })?;

  let path = Path::parse("local://test.png")?;
  let _bytes = path.read_all_bytes()?;

  platform.run(|platform| {
    platform.graphics().clear_active_frame_buffer(Color::PINK);
  });

  Ok(())
}
