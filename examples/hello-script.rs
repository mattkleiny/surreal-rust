use surreal::{backends::sdl::*, common::*, graphics::*, scripting::*};

fn main() {
  let mut window = Window::new(WindowSettings {
    title: "Hello Script!".to_string(),
    ..Default::default()
  })
  .expect("Failed to create window");

  GraphicsServer::install(OpenGLGraphicsBackend::new(&window));

  let mut runtime = JavascriptRuntime::new();

  runtime.add_callback("clear_screen", |r: f32, g: f32, b: f32| {
    graphics().clear_color_buffer(Color::rgb(r, g, b));

    1 + 1
  });

  while window.update() {
    runtime.eval("clear_screen(1, 0, 1);").unwrap();
    window.present();
  }
}
