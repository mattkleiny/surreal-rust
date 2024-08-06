//! A simple 'hello-world' of the sdl backend.

use common::Color;
use graphics::graphics;
use surreal_backend_sdl::{Window, WindowSettings};

fn main() {
  let mut window = Window::new(WindowSettings {
    title: "Hello, SDL!".to_string(),
    width: 1920,
    height: 1080,
    initial_color: Color::WHITE,
    ..WindowSettings::default()
  })
  .expect("Failed to build main window");

  while window.update() {
    graphics().clear_color_buffer(Color::WHITE);

    window.present()
  }
}
