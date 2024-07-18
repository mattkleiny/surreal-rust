use surreal::{audio::*, backends::sdl::*, common::*, graphics::*};

fn main() {
  let mut window = Window::new(WindowSettings {
    title: "Hello World!".to_string(),
    ..Default::default()
  })
  .expect("Failed to create window");

  let mut clock = DeltaClock::default();
  let mut total_time = 0.0;

  let color1 = Color::random();
  let color2 = Color::random();

  let mut source = AudioSource::new();
  let clip = AudioClip::from_wav_path("assets/audio/test-sound-1.wav").unwrap();

  source.play(&clip);

  while window.update() {
    let delta_time = clock.tick();
    total_time += delta_time;

    graphics().clear_color_buffer(Color::lerp(color1, color2, total_time.ping_pong()));

    window.present();
  }
}
