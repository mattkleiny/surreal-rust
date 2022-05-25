//! A fun little falling sand simulation for Surreal.

#![windows_subsystem = "windows"]

use surreal::prelude::*;

fn main() {
  let platform = DesktopPlatform::new(Configuration {
    title: "Falling Sand",
    ..Default::default()
  });

  Game::start(platform, |mut game| {
    let shader = load_standard_shader(&game.host.graphics);
    let palette = load_standard_palette(BuiltInPalette::Hollow4);

    let mut material = Material::new(&game.host.graphics, &shader);
    let mut canvas = PixelCanvas::new(&game.host.graphics, 256, 144);
    let mut random = Random::new();

    material.set_uniform("u_texture", &canvas.texture);
    material.set_uniform("u_projectionView", &Matrix4x4::identity());

    canvas.clear();

    game.run_variable_step(|context| {
      context.host.graphics.clear_color_buffer(palette[0]);

      canvas.simulate(context.time.delta_time);
      canvas.draw(&material);

      if let Some(keyboard) = context.host.input.primary_keyboard_device() {
        if keyboard.is_key_pressed(Key::Escape) {
          context.exit();
        }

        if keyboard.is_key_pressed(Key::Space) {
          canvas.clear();
        }
      }

      if let Some(mouse) = context.host.input.primary_mouse_device() {
        if mouse.is_button_down(MouseButton::Left) {
          let colors = &palette.as_slice()[1..4];
          let color = colors.select_randomly(&mut random);

          canvas.draw_circle(mouse.normalised_position(), 6., *color);
        } else if mouse.is_button_down(MouseButton::Right) {
          canvas.draw_circle(mouse.normalised_position(), 6., Color::CLEAR);
        }
      }
    });
  });
}