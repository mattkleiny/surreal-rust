//! A simple Hello, World for Surreal.

#![windows_subsystem = "windows"]

use surreal::prelude::*;

const WIDTH: f32 = 1920.;
const HEIGHT: f32 = 1080.;

fn main() {
  let platform = DesktopPlatform::new(Configuration {
    title: "Hello, World!",
    size: (WIDTH as u32, HEIGHT as u32),
    ..Default::default()
  });

  Game::start(platform, |mut game, assets| {
    let color1 = Color::random();
    let color2 = Color::random();

    let font: &BitmapFont = assets.load_asset("assets/fonts/IBM.font").expect("Failed to load font");
    let mut renderer = RenderManager::new(&game.host.graphics);

    renderer.configure(SpriteBatchDescriptor {
      projection_view: Matrix4x4::create_orthographic(WIDTH, HEIGHT, 0., 100.),
      ..Default::default()
    });

    game.run_variable_step(|context| {
      let total_time = context.time.total_time as f32;
      let color = Color::lerp(color1, color2, (total_time.sin() + 1.) / 2.);

      context.host.graphics.clear_color_buffer(color);

      renderer.with(|pass: &mut SpriteBatchContext| {
        font.draw_text(&mut pass.batch, "Hello, World!", vec2(WIDTH / 2., HEIGHT / 2.), Color::WHITE);
      });

      if let Some(keyboard) = context.host.input.keyboard_device() {
        if keyboard.is_key_pressed(Key::Escape) {
          context.exit();
        }
      }
    });
  });
}
