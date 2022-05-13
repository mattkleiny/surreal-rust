use surreal::prelude::*;

fn main() {
  let mut platform = DesktopPlatform::new(Configuration {
    title: "Falling Sand",
    ..Default::default()
  });

  let mut pixels = vec![Color::BLACK; 256 * 256];
  let mut pixels = GridSlice::new(&mut pixels, 256);

  platform.run(move |_platform| {
    pixels.fill(Color::WHITE);
  });
}
