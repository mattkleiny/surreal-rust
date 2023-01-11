//! An example of the WGPU [`surreal_graphics::GraphicsServerBackend`].

use winit::{
  dpi::PhysicalSize,
  event::{Event, WindowEvent},
  event_loop::EventLoop,
  window::WindowBuilder,
};

use surreal::utilities::DeltaClock;
use surreal_graphics::{CommandBuffer, GraphicsBackendKind};

fn main() {
  let event_loop = EventLoop::new();
  let window = WindowBuilder::new()
    .with_inner_size(PhysicalSize::new(1920, 1080))
    .with_title("Hello, WGPU!")
    .build(&event_loop)
    .unwrap();

  let graphics = pollster::block_on(surreal_graphics::GraphicsServer::from_kind(GraphicsBackendKind::WGPU, &window)).unwrap();

  let mut commands = CommandBuffer::default();
  let mut delta_clock = DeltaClock::new();

  macro_rules! attempt {
    ($body:expr) => {
      if let Err(error) = $body {
        surreal::diagnostics::error!("{}", error);
      }
    };
  }

  event_loop.run(move |event, _, control_flow| match event {
    Event::RedrawRequested(window_id) => {
      if window_id == window.id() {
        attempt!(graphics.execute_commands(&mut commands));
      }
    }
    Event::MainEventsCleared => {
      delta_clock.tick();

      window.request_redraw();
      *control_flow = winit::event_loop::ControlFlow::Poll;
    }
    Event::WindowEvent { event, .. } => match event {
      WindowEvent::Resized(new_size) => {
        attempt!(graphics.resize_viewport(new_size));
      }
      WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
        attempt!(graphics.resize_viewport(*new_inner_size));
      }
      WindowEvent::CloseRequested => *control_flow = winit::event_loop::ControlFlow::Exit,
      _ => (),
    },
    _ => (),
  });
}
