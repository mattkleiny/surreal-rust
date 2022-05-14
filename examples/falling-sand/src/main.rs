use surreal::prelude::*;

fn main() {
  let mut platform = DesktopPlatform::new(Configuration {
    title: "Falling Sand",
    ..Default::default()
  });

  let mut pixels = vec![Color::BLACK; 256 * 256];
  let mut pixels = Grid::new(&mut pixels, 256);

  platform.run(move |_platform| {
    pixels.fill(Color::WHITE);
    // pixels.draw_circle(vec2(0., 0.), 4., Color::WHITE);
  });
}
