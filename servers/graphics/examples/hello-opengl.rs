//! An example of the OpenGL [`surreal_graphics::GraphicsServerBackend`].

#![cfg(feature = "backend-opengl")]

fn main() {
  let event_loop = winit::event_loop::EventLoop::new();
  let window = winit::window::WindowBuilder::new().with_title("Hello, OpenGL!");
  let _backend = surreal_graphics::GraphicsServer::from_opengl(window).unwrap();

  event_loop.run(move |event, _, control_flow| {
    *control_flow = winit::event_loop::ControlFlow::Wait;

    match event {
      winit::event::Event::WindowEvent { event, .. } => match event {
        winit::event::WindowEvent::CloseRequested => *control_flow = winit::event_loop::ControlFlow::Exit,
        _ => (),
      },
      _ => (),
    }
  });
}
