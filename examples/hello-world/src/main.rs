use surreal::prelude::*;

fn main() {
  let platform = DesktopPlatform::new(Configuration {
    title: "Hello, World!",
    ..Default::default()
  });

  Game::start(platform, |game| {
    let color1 = Color::random();
    let color2 = Color::random();

    game.run_variable_step(|game, time| unsafe {
      let total_time = time.total_time as f32;
      let color = Color::lerp(color1, color2, (total_time.sin() + 1.) / 2.);

      game.platform.graphics.clear_color_buffer(color);
    });
  });
}
