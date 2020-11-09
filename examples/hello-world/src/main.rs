use surreal::platform::desktop::*;
use surreal::prelude::*;

fn main() {
  let configuration = Configuration {
    title: "Hello, World!",
    size: (1920 / 2, 1080 / 2),
  };

  let platform = DesktopPlatform::new(configuration)
    .expect("Failed to create platform!");

  platform.run(|platform| {
    platform.graphics().clear_active_frame_buffer(Color::BLACK);
  });
}
