//! A fun little falling sand simulation for Surreal.

#![windows_subsystem = "windows"]

use surreal::prelude::*;

fn main() {
  let configuration = Configuration {
    title: "Falling Sand",
    ..Default::default()
  };

  Engine::start(configuration, |engine| {
    let graphics = &engine.graphics;

    // set-up rendering
    let palette = load_built_in_palette(BuiltInPalette::Hollow4);
    let mut canvas = PixelCanvas::new(graphics, 256, 144);
    let mut random = Random::new();

    engine.run_variable_step(|engine, tick| {
      let mouse = &engine.input.mouse;

      canvas.simulate(tick.time.delta_time);
      canvas.draw();

      let size = vec2(canvas.pixels.width() as f32, canvas.pixels.height() as f32);
      let position = mouse.normalised_position() * size;

      if mouse.is_button_down(MouseButton::Left) {
        let colors = &palette.as_slice()[1..4];
        let color = colors.select_randomly(&mut random);

        canvas.draw_circle(position, 6., *color);
      } else if mouse.is_button_down(MouseButton::Right) {
        canvas.draw_circle(position, 6., Color::CLEAR);
      }

      if engine.input.keyboard.is_key_pressed(Key::Space) {
        canvas.clear();
      }

      if engine.input.keyboard.is_key_pressed(Key::Escape) {
        tick.exit();
      }
    });
  });
}
