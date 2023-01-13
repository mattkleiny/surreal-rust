#![cfg(target_os = "windows")]

use surreal::graphics::TextureFormat;
use surreal_graphics::*;

#[test]
fn test_graphics_operations() {
  common::bootstrap(|graphics| {
    let texture_id = graphics
      .texture_create(&TextureDescriptor {
        label: None,
        size: (1, 1, 1),
        format: TextureFormat::RGBA8,
      })
      .unwrap();

    graphics.texture_delete(texture_id).unwrap();
  });
}

mod common {
  use winit::{dpi::PhysicalSize, event_loop::EventLoopBuilder, window::WindowBuilder};

  use super::*;

  pub fn bootstrap(body: impl Fn(&GraphicsServer) + 'static) {
    use winit::platform::{run_return::EventLoopExtRunReturn, windows::EventLoopBuilderExtWindows};

    let mut event_loop = EventLoopBuilder::new().with_any_thread(true).build();
    let window = WindowBuilder::new()
      .with_inner_size(PhysicalSize::new(1920, 1080))
      .with_title("Surreal Integration Test")
      .with_visible(false)
      .build(&event_loop)
      .unwrap();

    let graphics = pollster::block_on(GraphicsServer::from_wgpu(&window)).unwrap();
    let mut delta_clock = surreal::utilities::DeltaClock::new();

    event_loop.run_return(move |_, _, control_flow| {
      delta_clock.tick();
      window.request_redraw();

      body(&graphics);

      *control_flow = winit::event_loop::ControlFlow::Exit;
    });
  }
}
