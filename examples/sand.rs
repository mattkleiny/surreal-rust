//! A fun little falling sand simulation for Surreal.

use surreal::prelude::*;

fn main() {
  let configuration = Configuration {
    title: "Falling Sand",
    log_level: LevelFilter::Trace,
    transparent_window: true,
    ..Default::default()
  };

  Engine::start(configuration, |engine, _| {
    let graphics = &engine.graphics;

    // set-up rendering
    let palette = load_built_in_palette(BuiltInPalette::Hollow4);
    let mut canvas = PixelCanvas::new(graphics, 256, 144);
    let mut random = Random::new();
    let mut timer = IntervalTimer::new(TimeSpan::from_millis(10.));

    engine.run_variable_step(|engine, tick| {
      let graphics = &engine.graphics;
      let mouse = &engine.input.mouse;

      graphics.clear_color_buffer(Color::rgba(0.2, 0.2, 0.2, 0.8));

      if timer.tick(tick.time.delta_time) {
        simulate_sand(&mut canvas.pixels);
        timer.reset();
      }

      canvas.draw();

      let size = vec2(canvas.pixels.width() as f32, canvas.pixels.height() as f32);
      let position = mouse.normalised_position() * size;

      if mouse.is_button_down(MouseButton::Left) {
        let colors = &palette.as_slice()[1..4];
        let color = *colors.select_randomly(&mut random);

        canvas.pixels.draw(
          color,
          &Circle {
            center: vec2(position.x.floor() as isize, position.y.floor() as isize),
            radius: 6,
          },
        );
      } else if mouse.is_button_down(MouseButton::Right) {
        canvas.pixels.draw(
          Color32::CLEAR,
          &Circle {
            center: vec2(position.x.floor() as isize, position.y.floor() as isize),
            radius: 6,
          },
        );
      }

      if engine.input.keyboard.is_key_pressed(Key::Space) {
        canvas.pixels.fill(Color32::CLEAR);
      }

      if engine.input.keyboard.is_key_pressed(Key::Escape) {
        tick.exit();
      }
    });
  });
}

fn simulate_sand(pixels: &mut Grid<Color32>) {
  for y in (0..pixels.height()).rev() {
    for x in 0..pixels.width() {
      let pixel = pixels.get_unchecked((x, y));

      if pixel.a <= 0 {
        continue;
      }

      match () {
        _ if simulate_particle(pixels, (x, y), (x as isize, y as isize + 1)) => (),
        _ if simulate_particle(pixels, (x, y), (x as isize - 1, y as isize + 1)) => (),
        _ if simulate_particle(pixels, (x, y), (x as isize + 1, y as isize + 1)) => (),
        _ => {}
      }
    }
  }
}

fn simulate_particle(pixels: &mut Grid<Color32>, from_pos: (usize, usize), to_pos: (isize, isize)) -> bool {
  let (from_x, from_y) = from_pos;
  let (to_x, to_y) = to_pos;

  if to_x < 0 || to_x > (pixels.width() - 1) as isize {
    return false;
  }

  if to_y < 0 || to_y > (pixels.height() - 1) as isize {
    return false;
  }

  let to_x = to_x as usize;
  let to_y = to_y as usize;

  let target = pixels.get_unchecked((to_x, to_y));

  if target.a == 0 {
    let source = *pixels.get_unchecked((from_x, from_y));

    pixels.set((to_x, to_y), source);
    pixels.set((from_x, from_y), Color32::CLEAR);

    return true;
  }

  false
}
