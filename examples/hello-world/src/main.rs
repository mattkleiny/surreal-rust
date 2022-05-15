use surreal::prelude::*;

fn main() {
  let mut platform = DesktopPlatform::new(Configuration {
    title: "Hello, World!",
    ..Default::default()
  });

  platform.run(|platform| unsafe {
    platform.graphics_server.clear_color_buffer(Color::BLACK);

    if let Some(keyboard) = platform.input_server.primary_keyboard_device() {
      if keyboard.is_key_pressed(Key::Escape) {
        platform.exit();
      }
    }
  });
}
