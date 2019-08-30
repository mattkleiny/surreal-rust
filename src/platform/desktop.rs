//! A desktop platform for Surreal.

use std::collections::HashSet;

use imgui::{im_str, Condition};
use sdl2::mouse::MouseState;
use sdl2::video::{GLContext, Window};
use sdl2::{AudioSubsystem, EventPump, Sdl, TimerSubsystem, VideoSubsystem};

use crate::audio::SoundClip;
use crate::graphics::Color;
use crate::input::Keycode;
use crate::timing::{Clock, DeltaTime, FpsCounter};

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
  pub max_fps: u32,
}

impl Platform for DesktopPlatform {
  type Host = DesktopHost;
  type Allocator = PortableAllocator;
  type FileSystem = PortableFileSystem;
  type AudioDevice = DesktopHost;
  type GraphicsDevice = DesktopHost;
  type InputDevice = DesktopHost;

  fn build(&self) -> Result<Self::Host, PlatformError> {
    Ok(DesktopHost::new(self.configuration, self.max_fps)?)
  }
}

/// A host for the desktop platform.
pub struct DesktopHost {
  sdl_context: Sdl,
  gl_context: GLContext,
  audio_subsystem: AudioSubsystem,
  video_subsystem: VideoSubsystem,
  timer_subsystem: TimerSubsystem,
  window: Window,
  imgui_context: imgui::Context,
  imgui_renderer: imgui_opengl_renderer::Renderer,
  imgui_sdl2: imgui_sdl2::ImguiSdl2,
  event_pump: EventPump,
  mouse_state: MouseState,
  keyboard_state: HashSet<Keycode>,
  max_frame_time: u32,
  is_closing: bool,
  render_debug_overlay: bool,
  delta_clock: Clock,
  fps_counter: FpsCounter,
}

impl DesktopHost {
  // TODO: properly implement the Result<T> types here
  pub fn new(configuration: WindowConfiguration, max_fps: u32) -> Result<Self, PlatformError> {
    let sdl_context = sdl2::init().map_err(|err| PlatformError::Initialization(err))?;
    let audio_subsystem = sdl_context.audio().map_err(|err| PlatformError::Initialization(err))?;
    let video_subsystem = sdl_context.video().map_err(|err| PlatformError::Initialization(err))?;
    let timer_subsystem = sdl_context.timer().map_err(|err| PlatformError::Initialization(err))?;

    // set the desired gl version before creating the window
    {
      let attr = video_subsystem.gl_attr();
      attr.set_context_profile(sdl2::video::GLProfile::Core);
      attr.set_context_version(3, 1);
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
      .unwrap();

    let event_pump = sdl_context.event_pump().unwrap();

    // prepare the opengl bindings and context
    let gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _);

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
      gl_context,
      audio_subsystem,
      video_subsystem,
      timer_subsystem,
      window,
      imgui_context,
      imgui_renderer,
      imgui_sdl2,
      event_pump,
      keyboard_state,
      mouse_state,
      max_frame_time: 1000 / max_fps,
      is_closing: false,
      render_debug_overlay: true,
      delta_clock: Clock::new(32.),
      fps_counter: FpsCounter::new(100),
    })
  }
}

impl DesktopHost {
  /// Sets the title of the window.
  pub fn set_title(&mut self, title: &String) {
    self.window.set_title(title.as_str()).unwrap();
  }
}

impl Host for DesktopHost {
  fn width(&self) -> u32 {
    self.window.size().0
  }
  fn height(&self) -> u32 {
    self.window.size().1
  }

  fn is_closing(&self) -> bool {
    self.is_closing
  }

  fn tick<C>(&mut self, mut callback: C)
  where
    C: FnMut(&mut Self, DeltaTime) -> (),
  {
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

    callback(self, delta_time);

    // prepare the imgui frame and render the debug overlay
    if self.render_debug_overlay {
      // prepare frame, transfer delta time to the ui
      self
        .imgui_sdl2
        .prepare_frame(self.imgui_context.io_mut(), &self.window, &self.mouse_state);
      self.imgui_context.io_mut().delta_time = delta_time as f32;

      let ui = self.imgui_context.frame();
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
          ui.text("Performance");
          ui.separator();
          ui.text(format!("Frames per second: {:.2}", frames_per_second));
        });

      ui.show_demo_window(&mut true);

      // render the frame
      self.imgui_sdl2.prepare_render(&ui, &self.window);
      self.imgui_renderer.render(ui);
    }

    // present to the window
    self.window.gl_swap_window();

    // don't eat the CPU; cap the FPS
    let frame_end = self.timer_subsystem.ticks();
    let frame_time = frame_end - frame_start;

    if frame_time < self.max_frame_time {
      self.timer_subsystem.delay(self.max_frame_time - frame_time);
    }

    self.fps_counter.tick(delta_time);
  }

  fn exit(&mut self) {
    self.is_closing = true;
  }
}

impl AudioDevice for DesktopHost {
  fn play(&mut self, _clip: &SoundClip) {
    unimplemented!()
  }
}

impl GraphicsDevice for DesktopHost {
  fn clear(&mut self, color: Color) {
    unsafe {
      gl::ClearColor(
        (color.r / 255) as f32,
        (color.g / 255) as f32,
        (color.b / 255) as f32,
        (color.a / 255) as f32,
      );
      gl::Clear(gl::COLOR_BUFFER_BIT);
    }
  }
}

impl InputDevice for DesktopHost {
  fn is_pressed(&self, binding: impl Into<Keycode>) -> bool {
    self.keyboard_state.contains(&binding.into())
  }
}
