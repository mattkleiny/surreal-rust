//! A desktop platform for Surreal.

use std::collections::HashSet;

use imgui::{Condition, im_str};
use sdl2::{AudioSubsystem, EventPump, Sdl, TimerSubsystem, VideoSubsystem};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseState;
use sdl2::video::Window;

use crate::graphics::{ClearOps, Color, GraphicsDevice};
use crate::graphics::opengl::OpenGLGraphicsDevice;
use crate::utilities::{Clock, FpsCounter};

use super::*;

/// The configuration for a window.
#[derive(Copy, Clone, Debug)]
pub struct WindowConfiguration {
  pub title: &'static str,
  pub width: u32,
  pub height: u32,
  pub show_cursor: bool,
}

/// An abstraction over the desktop platform.
pub struct DesktopPlatform {
  pub configuration: WindowConfiguration,
  pub max_fps: Option<u32>,
  pub use_vsync: bool,
  pub background_color: Color,
}

impl Platform for DesktopPlatform {
  type Host = DesktopHost;
  type GraphicsDevice = OpenGLGraphicsDevice;

  fn build(&self) -> Result<Self::Host, Error> {
    let host = DesktopHost::new(
      self.configuration,
      self.max_fps,
      self.use_vsync,
      self.background_color,
    );

    Ok(host?)
  }
}

/// A host for the desktop platform.
pub struct DesktopHost {
  sdl_context: Sdl,
  audio_subsystem: AudioSubsystem,
  video_subsystem: VideoSubsystem,
  timer_subsystem: TimerSubsystem,
  window: Window,
  graphics_device: OpenGLGraphicsDevice,
  imgui_context: imgui::Context,
  imgui_renderer: imgui_opengl_renderer::Renderer,
  imgui_sdl2: imgui_sdl2::ImguiSdl2,
  event_pump: EventPump,
  mouse_state: MouseState,
  keyboard_state: HashSet<Keycode>,
  max_frame_time: Option<u32>,
  is_closing: bool,
  render_debug_overlay: bool,
  delta_clock: Clock,
  fps_counter: FpsCounter,
  background_color: Color,
}

impl DesktopHost {
  pub fn new(configuration: WindowConfiguration, max_fps: Option<u32>, use_vsync: bool, background_color: Color) -> Result<Self, Error> {
    let sdl_context = sdl2::init().map_err(|err| Error::FailedToCreate(err))?;
    let audio_subsystem = sdl_context.audio().map_err(|err| Error::FailedToCreate(err))?;
    let video_subsystem = sdl_context.video().map_err(|err| Error::FailedToCreate(err))?;
    let timer_subsystem = sdl_context.timer().map_err(|err| Error::FailedToCreate(err))?;

    // set the desired gl version before creating the window
    {
      let attr = video_subsystem.gl_attr();
      attr.set_context_profile(sdl2::video::GLProfile::Core);
      attr.set_context_version(3, 2);
    }

    // prepare the main window and event pump
    let window = video_subsystem
        .window(
          configuration.title,
          configuration.width,
          configuration.height,
        )
        .position_centered()
        .resizable()
        .opengl()
        .allow_highdpi()
        .build()
        .map_err(|err| Error::FailedToCreate(err.to_string()))?;

    let event_pump = sdl_context.event_pump().map_err(|err| Error::FailedToCreate(err))?;

    // prepare the opengl bindings and context
    let gl_context = window.gl_create_context().map_err(|err| Error::FailedToCreate(err))?;
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _);

    // build the graphics device
    let graphics_device = OpenGLGraphicsDevice::new(gl_context, 0);

    // toggle vsync based on setting
    let vsync_enabled = if use_vsync { 1 } else { 0 };
    video_subsystem.gl_set_swap_interval(vsync_enabled).map_err(|err| Error::FailedToCreate(err))?;

    // prepare dear imgui for debug overlays
    let mut imgui_context = imgui::Context::create();
    let imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui_context, &window);

    let imgui_renderer = imgui_opengl_renderer::Renderer::new(&mut imgui_context, |s| {
      video_subsystem.gl_get_proc_address(s) as _
    });

    // capture the initial input device state
    let mouse_state = event_pump.mouse_state();
    let keyboard_state = event_pump
        .keyboard_state()
        .pressed_scancodes()
        .filter_map(Keycode::from_scancode)
        .collect();

    // toggle mouse cursor visibility
    if !configuration.show_cursor {
      sdl_context.mouse().show_cursor(configuration.show_cursor);
    }

    Ok(Self {
      sdl_context,
      audio_subsystem,
      video_subsystem,
      timer_subsystem,
      window,
      graphics_device,
      imgui_context,
      imgui_renderer,
      imgui_sdl2,
      event_pump,
      keyboard_state,
      mouse_state,
      max_frame_time: max_fps.map(|max_fps| 1000 / max_fps),
      is_closing: false,
      render_debug_overlay: true,
      delta_clock: Clock::new(32.),
      fps_counter: FpsCounter::new(100),
      background_color,
    })
  }
}

impl Host<DesktopPlatform> for DesktopHost {
  fn graphics_device(&self) -> &OpenGLGraphicsDevice {
    &self.graphics_device
  }

  fn is_closing(&self) -> bool {
    self.is_closing
  }

  fn tick<C>(&mut self, mut callback: C)
    where C: FnMut(&mut Self, f32) -> () {
    // pump window events for the SDL2 window
    for event in self.event_pump.poll_iter() {
      use sdl2::event::Event;

      match event {
        Event::KeyDown {
          keycode: Some(key), ..
        } => match key {
          Keycode::Escape => {
            self.is_closing = true;
          }
          _ => {}
        },
        Event::Quit { .. } => {
          self.is_closing = true;
        }
        _ => {}
      }
    }

    // update the input device state
    self.mouse_state = self.event_pump.mouse_state();
    self.keyboard_state = self
        .event_pump
        .keyboard_state()
        .pressed_scancodes()
        .filter_map(Keycode::from_scancode)
        .collect();

    // compute the delta time using the timer subsystem
    let frame_start = self.timer_subsystem.ticks();
    let delta_time = self.delta_clock.tick(
      self.timer_subsystem.performance_counter(),
      self.timer_subsystem.performance_frequency(),
    );

    // prepare the next frame for rendering
    unsafe {
      self.graphics_device.clear_render_target(&ClearOps {
        color: Some(self.background_color),
        depth: None,
        stencil: None,
      });
    }

    // tick the game simulation
    callback(self, delta_time);

    // prepare the imgui frame and render the debug overlay
    if self.render_debug_overlay {
      // prepare frame, transfer delta time to the ui
      self.imgui_sdl2.prepare_frame(self.imgui_context.io_mut(), &self.window, &self.mouse_state);
      self.imgui_context.io_mut().delta_time = delta_time as f32;

      let ui = self.imgui_context.frame();

      let average_frame_time = self.fps_counter.average_frame_time();
      let frames_per_second = self.fps_counter.fps();

      // build the debug overlay
      ui.window(im_str!("Debug Overlay"))
          .title_bar(false)
          .resizable(false)
          .always_auto_resize(true)
          .movable(false)
          .save_settings(false)
          .position([16., 16.], Condition::Always)
          .build(|| {
            ui.text("Statistics");
            ui.separator();
            ui.text(format!("Average frame time: {:.2}", average_frame_time));
            ui.text(format!("Frames per second: {:.2}", frames_per_second));

            // TODO: plot frame times here
          });

      // render the frame
      self.imgui_sdl2.prepare_render(&ui, &self.window);
      self.imgui_renderer.render(ui);
    }

    // finish rendering
    unsafe {
      self.graphics_device.flush_commands();
    }

    // present to the window
    self.window.gl_swap_window();

    // don't eat the CPU; cap the FPS
    if let Some(max_frame_time) = self.max_frame_time {
      let frame_end = self.timer_subsystem.ticks();
      let frame_time = frame_end - frame_start;

      if frame_time < max_frame_time {
        self.timer_subsystem.delay(max_frame_time - frame_time);
      }
    }

    self.fps_counter.tick(delta_time);
  }

  fn exit(&mut self) {
    self.is_closing = true;
  }
}
