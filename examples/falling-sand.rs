use surreal::backends::sdl::*;

fn main() {
  let mut window = Window::new(WindowSettings {
    title: "Falling Sand",
    ..Default::default()
  })
  .expect("Failed to create window");

  while window.update() {
    window.present();
  }
}
