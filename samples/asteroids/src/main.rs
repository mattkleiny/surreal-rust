//! A simple Asteroids game using Surreal.

use surreal::{
  backends::sdl::{Window, WindowSettings},
  graphics::{Color, GraphicsEngine},
};

fn main() {
  let mut window = Window::new(WindowSettings {
    title: "Asteroids".to_string(),
    width: 800,
    height: 600,
    vsync_enabled: true,
    icon: None,
  })
  .unwrap();

  let graphics = GraphicsEngine::opengl(&window);

  while window.update() {
    graphics.clear_color_buffer(Color::WHITE);

    window.present();
  }
}
