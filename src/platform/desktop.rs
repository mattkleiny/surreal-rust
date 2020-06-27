//! A platform implementation for desktop PCs.

use std::collections::HashSet;

use sdl2::{EventPump, Sdl, VideoSubsystem};
use sdl2::event::Event;
use sdl2::video::{GLContext, Window};

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
  // core state
  context: Sdl,
  gl_context: GLContext,
  event_pump: EventPump,

  // graphics and rendering
  video: VideoSubsystem,
  window: Window,

  // state management
  pressed_keys: HashSet<sdl2::keyboard::Keycode>,
}

impl DesktopPlatform {
  pub fn new(config: Configuration) -> Result<Self, PlatformError> {
    let context = sdl2::init()?;
    let video = context.video()?;

    let window = video.window(config.title, config.size.0, config.size.1)
        .position_centered()
        .resizable()
        .opengl()
        .allow_highdpi()
        .build()?;

    let gl_context = window.gl_create_context()?;
    gl::load_with(|s| video.gl_get_proc_address(s) as _);

    let event_pump = context.event_pump()?;

    Ok(DesktopPlatform {
      context,
      gl_context,
      event_pump,

      video,
      window,

      pressed_keys: HashSet::new(),
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

  fn run(mut self, mut callback: impl FnMut(&mut Self) -> bool) {
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

      if !callback(&mut self) {
        break 'running;
      }

      self.window.gl_swap_window();
    }
  }
}

impl From<String> for PlatformError {
  fn from(_: String) -> Self {
    PlatformError::General
  }
}

impl From<sdl2::video::WindowBuildError> for PlatformError {
  fn from(_: sdl2::video::WindowBuildError) -> Self {
    PlatformError::General
  }
}

impl From<sdl2::IntegerOrSdlError> for PlatformError {
  fn from(_: sdl2::IntegerOrSdlError) -> Self {
    PlatformError::General
  }
}
