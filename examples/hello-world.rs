use surreal::graphics::{Window, WindowSettings};

fn main() -> surreal::core::Result<()> {
  let window = Window::new(&WindowSettings {
    title: "Hello World!",
    ..Default::default()
  })?;

  while window.update() {
    window.present();
  }

  Ok(())
}
