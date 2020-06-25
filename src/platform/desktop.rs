//! A platform implementation for desktop PCs.

use sdl2::{EventPump, Sdl, VideoSubsystem};
use sdl2::event::Event;
use sdl2::render::WindowCanvas;

use crate::audio::*;
use crate::graphics::*;
use crate::input::*;
use crate::RID;

use super::*;

pub struct Configuration {
  pub title: &'static str,
  pub size: (u32, u32),
}

pub struct DesktopPlatform {
  context: Sdl,
  video: VideoSubsystem,
  canvas: WindowCanvas,
  event_pump: EventPump,
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
      video,
      canvas,
      event_pump,
    })
  }
}

impl Platform for DesktopPlatform {
  type Audio = DesktopPlatform;
  type Graphics = DesktopPlatform;
  type Input = DesktopPlatform;

  fn run(&mut self, mut callback: impl FnMut(&mut Self)) {
    'running: loop {
      for event in self.event_pump.poll_iter() {
        match event {
          Event::Quit { .. } => break 'running,
          _ => {}
        }
      };

      self.canvas.clear();
      callback(self);
      self.canvas.present();
    }
  }

  fn audio(&mut self) -> &mut Self::Audio { self }
  fn graphics(&mut self) -> &mut Self::Graphics { self }
  fn input(&mut self) -> &mut Self::Input { self }
}

impl AudioServer for DesktopPlatform {
  fn create_audio_source(&mut self) -> Result<RID, AudioSourceError> {
    unimplemented!()
  }
}

impl GraphicsServer for DesktopPlatform {
  fn clear(&mut self, color: Color) {
    let color: (u8, u8, u8, u8) = color.into();

    self.canvas.set_draw_color(color);
    self.canvas.clear();
  }

  fn create_texture(&mut self) -> Result<RID, TextureError> {
    Ok(RID(0)) // TODO: implement me
  }

  fn create_texture_from_image<P>(&mut self, image: &Image<P>) -> Result<RID, TextureError> {
    unimplemented!()
  }

  fn upload_texture_data<P>(&mut self, id: RID, image: &Image<P>) -> Result<(), TextureError> {
    unimplemented!()
  }
}

impl InputServer for DesktopPlatform {}

impl From<String> for PlatformError {
  fn from(_: String) -> Self {
    Self::FailedToCreate
  }
}

impl From<sdl2::video::WindowBuildError> for PlatformError {
  fn from(_: sdl2::video::WindowBuildError) -> Self {
    Self::FailedToCreate
  }
}

impl From<sdl2::IntegerOrSdlError> for PlatformError {
  fn from(_: sdl2::IntegerOrSdlError) -> Self {
    Self::FailedToCreate
  }
}