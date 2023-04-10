//! A fun little fractal generawtor using Surreal.

use surreal::{prelude::*, prototype::*};

const MAX_ITERATIONS: u32 = 25;
const CONSTANT_STEP: f32 = 0.003;

fn main() {
  EngineBuilder::default()
    .with_title("Fractals")
    .with_log_level(LevelFilter::Trace)
    .with_update_in_background(true)
    .start(|engine, _| {
      let graphics = &engine.graphics;

      // set-up rendering
      let palette: ColorPalette<Color32> = load_built_in_palette(BuiltInPalette::Ayy4);
      let mut canvas = PixelCanvas::new(graphics, 256, 144);
      let mut constant = vec2(0.285, 0.01);

      engine.run_variable_step(|engine, time| {
        engine
          .graphics
          .clear_color_buffer(Color::rgb(0.1, 0.1, 0.1));

        render(&mut canvas, &palette, constant);
        constant += vec2(0.001, 0.0) * time.delta_time;

        canvas.draw();

        if let Some(keyboard) = &engine.input.keyboard {
          if keyboard.is_key_pressed(Key::Up) {
            constant.y += CONSTANT_STEP;
          }

          if keyboard.is_key_pressed(Key::Down) {
            constant.y -= CONSTANT_STEP;
          }

          if keyboard.is_key_pressed(Key::Left) {
            constant.x -= CONSTANT_STEP;
          }

          if keyboard.is_key_pressed(Key::Right) {
            constant.x += CONSTANT_STEP;
          }

          if keyboard.is_key_pressed(Key::Escape) {
            engine.quit();
          }
        }
      })
    })
    .expect("An unexpected error occurred");
}

fn render<P: Pixel + Texel>(
  canvas: &mut PixelCanvas<P>,
  palette: &ColorPalette<P>,
  constant: Vec2,
) {
  let scale = 1. / canvas.height() as f32 / 2.;

  for x in 0..canvas.width() as i32 {
    for y in 0..canvas.height() as i32 {
      let initial = Vec2::new(
        (x as f32 - canvas.width() as f32 / 2.) * scale,
        (y as f32 - canvas.height() as f32 / 2.) * scale,
      );

      let iterations = compute_iterations(initial, constant);
      let color = palette[iterations as usize % palette.len()];

      canvas.set_pixel(x, y, color);
    }
  }
}

fn compute_next(current: Vec2, constant: Vec2) -> Vec2 {
  let zr = current.x * current.x - current.y * current.y + constant.x;
  let zi = 2. * current.x * current.y + constant.y;

  Vec2::new(zr, zi)
}

fn compute_iterations(initial: Vec2, constant: Vec2) -> u32 {
  let mut current = initial;
  let mut iterations = 0;

  while mod2(current) < 4. && iterations < MAX_ITERATIONS {
    current = compute_next(current, constant);
    iterations += 1;
  }

  iterations
}

#[inline(always)]
fn mod2(current: Vec2) -> f32 {
  current.x * current.x + current.y * current.y
}
