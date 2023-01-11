//! An example of the WGPU [`surreal_graphics::GraphicsServerBackend`].

#![cfg(feature = "backend-wgpu")]

use std::sync::Arc;

use winit::{
  dpi::PhysicalSize,
  event::{Event, WindowEvent},
  event_loop::EventLoop,
  window::WindowBuilder,
};

use surreal::graphics::Color;
use surreal::maths::{FromRandom, Lerp};
use surreal::utilities::DeltaClock;

fn main() {
  let event_loop = EventLoop::new();
  let window = Arc::new(
    WindowBuilder::new()
      .with_inner_size(PhysicalSize::new(1920, 1080))
      .with_title("Hello, WGPU!")
      .build(&event_loop)
      .unwrap(),
  );

  let mut graphics = pollster::block_on(surreal_graphics::GraphicsServer::from_wgpu(window.clone())).unwrap();

  let color1 = Color::random();
  let color2 = Color::random();

  let mut delta_clock = DeltaClock::new();

  event_loop.run(move |event, _, control_flow| match event {
    Event::RedrawRequested(window_id) => {
      if window_id == window.id() {
        let color = Color::lerp(color1, color2, (delta_clock.total_time().sin() + 1.) / 2.);

        graphics.begin_frame(color).expect("Failed to begin frame");
        graphics.end_frame().expect("Failed to end frame");
      }
    }
    Event::MainEventsCleared => {
      delta_clock.tick();

      window.request_redraw();
      *control_flow = winit::event_loop::ControlFlow::Poll;
    }
    Event::WindowEvent { event, .. } => match event {
      WindowEvent::Resized(new_size) => {
        graphics.resize_viewport(new_size).expect("Failed to resize");
      }
      WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
        graphics.resize_viewport(*new_inner_size).expect("Failed to resize");
      }
      WindowEvent::CloseRequested => *control_flow = winit::event_loop::ControlFlow::Exit,
      _ => (),
    },
    _ => (),
  });
}
