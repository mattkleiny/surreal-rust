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

    // load assets
    let palette = load_standard_palette(BuiltInPalette::Hollow4);

    // set-up rendering
    let mut renderer = RenderManager::new(graphics);
    let mut canvas = PixelCanvas::new(graphics, 256, 144);
    let mut random = Random::new();

    renderer.configure(SpriteBatchDescriptor::default());

    game.run_variable_step(|context| {
      renderer.with(|pass: &mut SpriteBatchContext| {
        context.host.graphics.clear_color_buffer(palette[0]);

        canvas.simulate(context.time.delta_time);
        canvas.draw(&pass.material); // we're just using the sprite material
      });

      if let Some(keyboard) = context.host.input.keyboard_device() {
        if keyboard.is_key_pressed(Key::Escape) {
          context.exit();
        }

        if keyboard.is_key_pressed(Key::Space) {
          canvas.clear();
        }
      }

      if let Some(mouse) = context.host.input.mouse_device() {
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