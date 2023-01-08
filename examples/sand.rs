//! A fun little falling sand simulation for Surreal.

use surreal::prelude::*;
use surreal::prototype::*;

fn main() {
  let configuration = EngineConfig {
    title: "Falling Sand".to_string(),
    log_level: LevelFilter::Trace,
    transparent_window: true,
    ..Default::default()
  };

  Engine::start(configuration, |engine, _| {
    let graphics = &engine.graphics;

    // set-up rendering
    let palette = load_built_in_palette(BuiltInPalette::Hollow4);
    let mut canvas = PixelCanvas::new(graphics, 256, 144);
    let mut timer = IntervalTimer::new(TimeSpan::from_millis(10.));

    engine.run_variable_step(|engine, time| {
      let graphics = &engine.graphics;

      graphics.clear_color_buffer(Color::rgba(0.2, 0.2, 0.2, 0.8));

      if timer.tick(time.delta_time) {
        simulate_sand(&mut canvas.pixels);
        timer.reset();
      }

      canvas.draw();

      if let Some(mouse) = &engine.input.mouse {
        let size = vec2(canvas.pixels.width() as f32, canvas.pixels.height() as f32);
        let position = mouse.normalised_position() * size;

        if mouse.is_button_down(MouseButton::Left) {
          let colors = &palette.as_slice()[1..4];
          let color = colors[usize::random() % colors.len()];

          for offset_y in -2..2 {
            for offset_x in -2..2 {
              let x = offset_x + position.x as i32;
              let y = offset_y + position.y as i32;

              canvas.pixels.set(x, y, color);
            }
          }
        } else if mouse.is_button_down(MouseButton::Right) {
          for offset_y in -2..2 {
            for offset_x in -2..2 {
              let x = offset_x + position.x as i32;
              let y = offset_y + position.y as i32;

              canvas.pixels.set(x, y, Color32::CLEAR);
            }
          }
        }
      }

      if let Some(keyboard) = &engine.input.keyboard {
        if keyboard.is_key_pressed(Key::Space) {
          canvas.pixels.fill(Color32::CLEAR);
        }

        if keyboard.is_key_pressed(Key::Escape) {
          engine.quit();
        }
      }
    });
  });
}

fn simulate_sand(pixels: &mut Grid<Color32>) {
  for y in (0..pixels.height()).rev() {
    for x in 0..pixels.width() {
      let pixel = unsafe { pixels.get_unchecked(x as i32, y as i32) };

      if pixel.a <= 0 {
        continue;
      }

      match () {
        _ if simulate_particle(pixels, (x as i32, y as i32), (x as i32, y as i32 + 1)) => (),
        _ if simulate_particle(pixels, (x as i32, y as i32), (x as i32 - 1, y as i32 + 1)) => (),
        _ if simulate_particle(pixels, (x as i32, y as i32), (x as i32 + 1, y as i32 + 1)) => (),
        _ => {}
      }
    }
  }
}

fn simulate_particle(pixels: &mut Grid<Color32>, from_pos: (i32, i32), to_pos: (i32, i32)) -> bool {
  let (from_x, from_y) = from_pos;
  let (to_x, to_y) = to_pos;

  if to_x < 0 || to_x > (pixels.width() - 1) as i32 {
    return false;
  }

  if to_y < 0 || to_y > (pixels.height() - 1) as i32 {
    return false;
  }

  let to_x = to_x as i32;
  let to_y = to_y as i32;

  unsafe {
    let target = pixels.get_unchecked(to_x, to_y);

    if target.a == 0 {
      let source = *pixels.get_unchecked(from_x, from_y);

      pixels.set(to_x, to_y, source);
      pixels.set(from_x, from_y, Color32::CLEAR);

      return true;
    }
  }

  false
}
