//! A platform implementation for desktop PCs.

use winit::{
  dpi::LogicalSize,
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::{Window, WindowBuilder},
};

use crate::platform::{Platform, PlatformError};

mod audio;
mod graphics;
mod input;
mod window;

#[derive(Copy, Clone, Debug)]
pub struct Configuration {
  pub title: &'static str,
  pub size: (u32, u32),
}

pub struct DesktopPlatform {
  event_loop: Option<EventLoop<()>>,
  window: Window,
}

impl DesktopPlatform {
  pub fn new(config: Configuration) -> Result<Self, PlatformError> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(config.title)
        .with_inner_size(LogicalSize::new(config.size.0, config.size.1))
        .build(&event_loop)?;

    Ok(Self {
      event_loop: Some(event_loop),
      window,
    })
  }
}

impl Platform for DesktopPlatform {
  type Audio = Self;
  type Graphics = Self;
  type Input = Self;
  type Window = Self;

  fn audio(&mut self) -> &mut Self::Audio { self }
  fn graphics(&mut self) -> &mut Self::Graphics { self }
  fn input(&mut self) -> &mut Self::Input { self }
  fn window(&mut self) -> &mut Self::Window { self }

  fn run(mut self, callback: impl FnMut(&mut Self) -> bool) {
    let event_loop = self.event_loop.take().unwrap();

    event_loop.run(move |event, _, control_flow| {
      *control_flow = ControlFlow::Wait;

      match event {
        Event::WindowEvent { window_id, event } if window_id == self.window.id() => {
          match event {
            WindowEvent::CloseRequested => {
              *control_flow = ControlFlow::Exit;
            }
            _ => {}
          }
        }
        Event::RedrawRequested(window_id) if window_id == self.window.id() => {
          // callback(&mut self);
        }
        _ => {}
      }
    });
  }
}

impl From<winit::error::OsError> for PlatformError {
  fn from(_: winit::error::OsError) -> Self {
    PlatformError::General
  }
}