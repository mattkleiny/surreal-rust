use surreal::graphics::{Color, GraphicsEngine, Window, WindowSettings};

fn main() -> surreal::core::Result<()> {
  let window = Window::new(&WindowSettings {
    title: "Hello World!",
    ..Default::default()
  })?;

  let graphics = GraphicsEngine::create_opengl(&window);

  while window.update() {
    graphics.clear_color_buffer(Color::BLACK);
    window.present();
  }

  Ok(())
}
