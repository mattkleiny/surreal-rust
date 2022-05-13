use surreal::prelude::*;

fn main() {
  let mut platform = DesktopPlatform::new(Configuration {
    title: "Hello, World!",
    ..Default::default()
  });

  platform.run(|platform| unsafe {
    platform.graphics_server.clear_color_buffer(Color::MAGENTA);
  });
}
