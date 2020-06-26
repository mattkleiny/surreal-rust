//! A platform implementation for desktop PCs.

use std::collections::HashSet;

use sdl2::{EventPump, Sdl, VideoSubsystem};
use sdl2::event::Event;
use sdl2::render::WindowCanvas;

use super::*;

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
  // core state
  context: Sdl,
  event_pump: EventPump,

  // graphics and rendering
  video: VideoSubsystem,
  canvas: WindowCanvas,

  // input management
  pressed_keys: HashSet<sdl2::keyboard::Keycode>,
}

impl DesktopPlatform {
  pub fn new(config: Configuration) -> Result<Self, PlatformError> {
    let context = sdl2::init()?;
    let video = context.video()?;

    let window = video.window(config.title, config.size.0, config.size.1)
      .position_centered()
      .resizable()
      .build()?;

    let canvas = window.into_canvas()
      .present_vsync()
      .accelerated()
      .build()?;

    let event_pump = context.event_pump()?;

    Ok(DesktopPlatform {
      context,
      event_pump,
      video,
      canvas,
      pressed_keys: HashSet::new(),
    })
  }
}

impl Platform for DesktopPlatform {
  type Audio = DesktopPlatform;
  type Graphics = DesktopPlatform;
  type Input = DesktopPlatform;
  type Window = DesktopPlatform;

  fn audio(&mut self) -> &mut Self::Audio { self }
  fn graphics(&mut self) -> &mut Self::Graphics { self }
  fn input(&mut self) -> &mut Self::Input { self }
  fn window(&mut self) -> &mut Self::Window { self }

  fn run(&mut self, mut callback: impl FnMut(&mut Self) -> bool) {
    'running: loop {
      for event in self.event_pump.poll_iter() {
        match event {
          Event::Quit { .. } => break 'running,
          Event::KeyDown { keycode: Some(key), .. } => {
            self.pressed_keys.insert(key);
          }
          Event::KeyUp { keycode: Some(key), .. } => {
            self.pressed_keys.remove(&key);
          }
          _ => {}
        }
      };

      self.canvas.clear();

      if !callback(self) {
        break 'running;
      }

      self.canvas.present();
    }
  }
}

impl From<String> for PlatformError {
  fn from(_: String) -> Self {
    PlatformError::GeneralFailure
  }
}

impl From<sdl2::video::WindowBuildError> for PlatformError {
  fn from(_: sdl2::video::WindowBuildError) -> Self {
    PlatformError::FailedToCreate
  }
}

impl From<sdl2::IntegerOrSdlError> for PlatformError {
  fn from(_: sdl2::IntegerOrSdlError) -> Self {
    PlatformError::GeneralFailure
  }
}
