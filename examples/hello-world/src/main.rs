#![windows_subsystem = "windows"]

use surreal::prelude::*;

fn main() {
  let platform = DesktopPlatform::new(Configuration {
    title: "Hello, World!",
    ..Default::default()
  });

  Game::start(platform, |mut game| {
    let color1 = Color::random();
    let color2 = Color::random();

    let mut pixels = Vec::with_capacity(256 * 144);
    let mut texture = Texture::new_with_options(&game.host.graphics, TextureFormat::RGBA, TextureFilter::Nearest, TextureWrap::Clamp);

    pixels.fill(Color::WHITE);

    game.run_variable_step(|context| unsafe {
      let total_time = context.time.total_time as f32;
      let color = Color::lerp(color1, color2, (total_time.sin() + 1.) / 2.);

      texture.write_pixels(256, 144, pixels.as_slice());

      context.host.graphics.clear_color_buffer(color);

      if let Some(keyboard) = context.host.input.primary_keyboard_device() {
        if keyboard.is_key_pressed(Key::Escape) {
          context.exit();
        }
      }
    });
  });
}
