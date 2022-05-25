//! A fun little falling sand simulation for Surreal.

#![windows_subsystem = "windows"]

use surreal::prelude::*;

fn main() {
  let platform = DesktopPlatform::new(Configuration {
    title: "Falling Sand",
    ..Default::default()
  });

  Game::start(platform, |mut game| {
    let graphics = &game.host.graphics;

    let mut renderer = Renderer::new(graphics);
    let sprite_descriptor = SpriteContextDescriptor::default();
    let palette = load_standard_palette(BuiltInPalette::Hollow4);

    let mut canvas = PixelCanvas::new(graphics, 256, 144);
    let mut random = Random::new();

    game.run_variable_step(|context| {
      renderer.with(&sprite_descriptor, |pass| {
        context.host.graphics.clear_color_buffer(palette[0]);

        // TODO: draw to sprite batch, instead?
        canvas.simulate(context.time.delta_time);
        canvas.draw(&pass.material);
      });

      if let Some(keyboard) = context.host.input.primary_keyboard_device() {
        if keyboard.is_key_pressed(Key::Escape) {
          context.exit();
        }

        if keyboard.is_key_pressed(Key::Space) {
          canvas.clear();
        }
      }

      if let Some(mouse) = context.host.input.primary_mouse_device() {
        let position = mouse.normalised_position();

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