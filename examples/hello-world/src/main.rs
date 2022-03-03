use surreal::prelude::*;

fn main() {
  let configuration = Configuration {
    title: "Hello, World!",
    size: (1920 / 2, 1080 / 2),
  };

  let platform = DesktopPlatform::new(configuration)
      .expect("Failed to create platform!");

  let buffer = Buffer::new(
    platform.graphics_server.clone(),
    BufferKind::Element,
    BufferUsage::Static,
  );

  platform.run();
}
