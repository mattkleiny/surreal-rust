//! An example of headless bootstrapping for Surreal.

use surreal::prelude::*;

fn main() {
  let platform = HeadlessPlatform;

  Game::start(platform, |mut game| {
    let mut clock = Clock::new();
    let mut timer = IntervalTimer::new(TimeSpan::from_seconds(1.));
    let mut counter = FrameCounter::new(32);

    game.run_variable_step(|context| {
      let delta_time = clock.tick();

      counter.tick(delta_time);

      if timer.tick(delta_time) {
        timer.reset();

        println!("Frames per second {:.2}", counter.fps());
      }

      if let Some(keyboard) = context.host.input.keyboard_device() {
        if keyboard.is_key_pressed(Key::Escape) {
          context.exit();
        }
      }
    });
  });
}
