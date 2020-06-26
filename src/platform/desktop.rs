//! A platform implementation for desktop PCs.

use std::collections::HashSet;

use sdl2::{EventPump, Sdl, VideoSubsystem};
use sdl2::event::Event;
use sdl2::video::{GLContext, Window};

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
  gl_context: GLContext,
  event_pump: EventPump,

  // graphics and rendering
  video: VideoSubsystem,
  window: Window,

  // imgui
  imgui: imgui::Context,
  imgui_sdl2: imgui_sdl2::ImguiSdl2,
  imgui_renderer: imgui_opengl_renderer::Renderer,

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

    let mut imgui = imgui::Context::create();

    imgui.set_ini_filename(None);

    let imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &window);
    let imgui_renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| video.gl_get_proc_address(s) as _);

    let event_pump = context.event_pump()?;

    Ok(DesktopPlatform {
      context,
      gl_context,
      event_pump,

      video,
      window,

      imgui,
      imgui_sdl2,
      imgui_renderer,

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

  fn run(&mut self, mut callback: impl FnMut(&mut Self) -> bool) {
    'running: loop {
      for event in self.event_pump.poll_iter() {
        self.imgui_sdl2.handle_event(&mut self.imgui, &event);
        if self.imgui_sdl2.ignore_event(&event) { continue; }

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

      self.imgui_sdl2.prepare_frame(
        self.imgui.io_mut(),
        &self.window,
        &self.event_pump.mouse_state(),
      );

      if !callback(self) {
        break 'running;
      }

      let frame = self.imgui.frame();

      frame.plot_histogram(im_str!("FPS"), &[0.25; 32])
          .scale_min(0.)
          .scale_max(100.)
          .graph_size([100., 100.])
          .build();

      self.imgui_sdl2.prepare_render(&frame, &self.window);
      self.imgui_renderer.render(frame);

      self.window.gl_swap_window();
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
