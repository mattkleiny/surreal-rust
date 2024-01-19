//! The main editor binary for the Surreal Project.

use sdl::{Window, WindowSettings};

fn main() {
  let window = Window::new(&WindowSettings {
    title: "Surreal Editor",
    ..Default::default()
  })
  .unwrap();

  while window.update() {
    window.present();
  }
}
