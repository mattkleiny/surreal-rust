//! An example of the Vulkan [`surreal_graphics::GraphicsServerBackend`].

#![cfg(feature = "backend-vulkan")]

use std::sync::Arc;

use winit::{
  dpi::PhysicalSize,
  event::{Event, WindowEvent},
  event_loop::EventLoop,
  window::WindowBuilder,
};

fn main() {
  let event_loop = EventLoop::new();
  let window = Arc::new(
    WindowBuilder::new()
      .with_inner_size(PhysicalSize::new(1920, 1080))
      .with_title("Hello, Vulkan!")
      .build(&event_loop)
      .unwrap(),
  );

  let graphics = surreal_graphics::GraphicsServer::from_vulkan(window.clone()).unwrap();

  event_loop.run(move |event, _, control_flow| {
    *control_flow = winit::event_loop::ControlFlow::Wait;

    match event {
      Event::RedrawRequested(_) => {
        graphics.begin_frame();

        graphics.end_frame();
      }
      Event::WindowEvent { event, .. } => match event {
        WindowEvent::CloseRequested => *control_flow = winit::event_loop::ControlFlow::Exit,
        _ => (),
      },
      _ => (),
    }
  });
}
