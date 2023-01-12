//! An example of the WGPU [`surreal_graphics::GraphicsBackend`].

use surreal::graphics::TextureFormat;
use winit::{
  dpi::PhysicalSize,
  event::{Event, WindowEvent},
  event_loop::EventLoop,
  window::WindowBuilder,
};

use surreal::utilities::DeltaClock;
use surreal_graphics::{urp::*, *};

fn main() {
  // build window
  let event_loop = EventLoop::new();
  let window = WindowBuilder::new()
    .with_inner_size(PhysicalSize::new(1920, 1080))
    .with_title("Hello, WGPU!")
    .build(&event_loop)
    .unwrap();

  // build graphics server and pipeline
  let graphics = pollster::block_on(GraphicsServer::from_wgpu(&window)).unwrap();
  let mut manager = UniversalPipeline::new(&graphics).unwrap();
  let mut delta_clock = DeltaClock::new();

  let texture = graphics
    .texture_create(&TextureDescriptor {
      label: Some("Test texture"),
      size: (16, 16, 0),
      format: TextureFormat::RGBA8,
    })
    .unwrap();

  graphics.texture_delete(texture).unwrap();

  event_loop.run(move |event, _, control_flow| match event {
    Event::RedrawRequested(window_id) => {
      if window_id == window.id() {
        manager.begin_frame();
        manager.end_frame().unwrap();
      }
    }
    Event::MainEventsCleared => {
      delta_clock.tick();

      window.request_redraw();
      *control_flow = winit::event_loop::ControlFlow::Poll;
    }
    Event::WindowEvent { event, .. } => match event {
      WindowEvent::Resized(new_size) => {
        graphics.resize_viewport(new_size).unwrap();
      }
      WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
        graphics.resize_viewport(*new_inner_size).unwrap();
      }
      WindowEvent::CloseRequested => *control_flow = winit::event_loop::ControlFlow::Exit,
      _ => (),
    },
    _ => (),
  });
}
