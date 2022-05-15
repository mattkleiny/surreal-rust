use surreal::prelude::*;

fn main() {
  let mut platform = DesktopPlatform::new(Configuration {
    title: "Hello, World!",
    ..Default::default()
  });

  let color1 = Color::random();
  let color2 = Color::random();

  let mut timer = Clock::new();
  let mut total_time = 0.;

  platform.run(|platform| unsafe {
    total_time += timer.tick() as f32;

    if let Some(keyboard) = platform.input.primary_keyboard_device() {
      if keyboard.is_key_pressed(Key::Escape) {
        platform.exit();
      }
    }

    platform.graphics.clear_color_buffer(Color::lerp(color1, color2, (total_time.sin() + 1.) / 2.));
  });
}
