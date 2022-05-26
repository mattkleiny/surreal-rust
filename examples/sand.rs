//! A fun little falling sand simulation for Surreal.

#![windows_subsystem = "windows"]

use surreal::prelude::*;

fn main() {
  let platform = DesktopPlatform::new(Configuration {
    title: "Falling Sand",
    ..Default::default()
  });

  Game::start(platform, |mut game, _| {
    let graphics = &game.host.graphics;

    // set-up rendering
    let palette = load_built_in_palette(BuiltInPalette::Hollow4);
    let mut canvas = PixelCanvas::new(graphics, 256, 144);
    let mut random = Random::new();

    game.run_variable_step(|context| {
      canvas.simulate(context.time.delta_time);
      canvas.draw(); // we're just using the sprite material

      if let Some(keyboard) = context.host.input.keyboard_device() {
        if keyboard.is_key_pressed(Key::Escape) {
          context.exit();
        }

        if keyboard.is_key_pressed(Key::Space) {
          canvas.clear();
        }
      }

      if let Some(mouse) = context.host.input.mouse_device() {
        let position = mouse.normalised_position() * vec2(canvas.pixels.width() as f32, canvas.pixels.height() as f32);

        if mouse.is_button_down(MouseButton::Left) {
          let colors = &palette.as_slice()[1..4];
          let color = colors.select_randomly(&mut random);

          canvas.draw_circle(position, 6., *color);
        } else if mouse.is_button_down(MouseButton::Right) {
          canvas.draw_circle(position, 6., Color::CLEAR);
        }
      }
    });
  });
}