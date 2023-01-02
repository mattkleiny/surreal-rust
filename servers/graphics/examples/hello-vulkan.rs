//! An example of the Vulkan [`surreal_graphics::GraphicsServerBackend`].

fn main() {
  let event_loop = winit::event_loop::EventLoop::new();
  let window = winit::window::WindowBuilder::new()
    .with_title("Hello, Vulkan!")
    .build(&event_loop)
    .unwrap();

  let _backend = surreal_graphics::GraphicsServer::from_vulkan(&window).unwrap();

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
