//! An example of headless bootstrapping for Surreal.

use surreal::prelude::*;

fn main() {
  Game::start(HeadlessPlatform, |mut game, _| {
    let mut clock = Clock::new();
    let mut timer = IntervalTimer::new(TimeSpan::from_seconds(1.));
    let mut counter = FrameCounter::new(32);

    game.run_variable_step(|game| {
      let delta_time = clock.tick();

      counter.tick(delta_time);

      if timer.tick(delta_time) {
        timer.reset();

        println!("Frames per second {:.2}", counter.fps());
      }

      if let Some(keyboard) = game.host.input.keyboard_device() {
        if keyboard.is_key_pressed(Key::Escape) {
          game.exit();
        }
      }
    });
  });
}
