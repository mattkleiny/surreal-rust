//! A fun little falling sand simulation for Surreal.

use surreal::prelude::*;

fn main() {
  let configuration = Configuration {
    title: "Falling Sand",
    log_level: LevelFilter::Trace,
    transparent_window: true,
    ..Default::default()
  };

  Engine::start(configuration, |engine| {
    let graphics = &engine.graphics;

    // set-up rendering
    let palette = load_built_in_palette(BuiltInPalette::Hollow4);
    let mut canvas = PixelCanvas::new(graphics, 256, 144);
    let mut random = Random::new();

    engine.run_variable_step(|engine, tick| {
      let graphics = &engine.graphics;
      let mouse = &engine.input.mouse;

      graphics.clear_color_buffer(Color::rgba(0.2, 0.2, 0.2, 0.8));

      canvas.simulate(tick.time.delta_time);
      canvas.draw();

      let size = vec2(canvas.pixels.width() as f32, canvas.pixels.height() as f32);
      let position = mouse.normalised_position() * size;

      if mouse.is_button_down(MouseButton::Left) {
        let colors = &palette.as_slice()[1..4];
        let color = colors.select_randomly(&mut random);

        canvas.draw_circle(position, 6., *color);
      } else if mouse.is_button_down(MouseButton::Right) {
        canvas.draw_circle(position, 6., Color32::CLEAR);
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
