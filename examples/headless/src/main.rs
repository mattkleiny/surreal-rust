use surreal::prelude::*;

fn main() {
  let platform = HeadlessPlatform;

  let mut clock = Clock::new();
  let mut timer = IntervalTimer::new(TimeSpan::from_seconds(1.));
  let mut counter = FrameCounter::new(32);

  Game::start(platform, |mut game| {
    game.run_variable_step(|context| unsafe {
      let delta_time = clock.tick();

      counter.tick(delta_time);

      if timer.tick(delta_time) {
        timer.reset();

        println!("Frames per second {:.2}", counter.fps());
      }

      if let Some(keyboard) = context.host.input.primary_keyboard_device() {
        if keyboard.is_key_pressed(Key::Escape) {
          context.exit();
        }
      }
    });
  });
}
