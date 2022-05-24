//! A fun little falling sand simulation for Surreal.

#![windows_subsystem = "windows"]

use surreal::prelude::*;

const SHADER_CODE: &'static str = include_str!("../assets/shaders/standard.glsl");
const COLOR_PALETTE: &'static [u8] = include_bytes!("../assets/palettes/hollow-4.pal");

fn main() {
  let platform = DesktopPlatform::new(Configuration {
    title: "Falling Sand",
    ..Default::default()
  });

  Game::start(platform, |mut game| {
    let shader = ShaderProgram::from_string(&game.host.graphics, SHADER_CODE).expect("Failed to load standard shader");
    let palette = ColorPalette::from_reader(COLOR_PALETTE).expect("Failed to load palette");

    let mut material = Material::new(&game.host.graphics, &shader);
    let mut canvas = PixelCanvas::new(&game.host.graphics, 256, 144);

    material.set_uniform("u_texture", &canvas.texture);

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

          canvas.draw_circle(mouse.normalised_position(), 6., *colors.select_randomly());
        } else if mouse.is_button_down(MouseButton::Right) {
          canvas.draw_circle(mouse.normalised_position(), 6., Color::CLEAR);
        }
      }
    });
  });
}