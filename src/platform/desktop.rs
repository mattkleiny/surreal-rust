//! A desktop platform for Surreal.

use std::collections::HashSet;

use sdl2::{AudioSubsystem, EventPump, Sdl, TimerSubsystem, VideoSubsystem};
use sdl2::mouse::MouseState;
use sdl2::render::WindowCanvas;

use crate::audio::AudioClip;
use crate::graphics::Color;
use crate::input::Keycode;
use crate::timing::Clock;

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

  fn build(&self) -> Result<Self::Host> {
    Ok(DesktopHost::new(self.configuration, self.max_fps)?)
  }
}

/// A host for the desktop platform.
pub struct DesktopHost {
  sdl_context: Sdl,
  audio_subsystem: AudioSubsystem,
  video_subsystem: VideoSubsystem,
  timer_subsystem: TimerSubsystem,
  window_canvas: WindowCanvas,
  event_pump: EventPump,
  mouse_state: MouseState,
  keyboard_state: HashSet<Keycode>,
  max_frame_time: u32,
  is_closing: bool,
  delta_clock: Clock,
}

impl DesktopHost {
  // TODO: properly implement the Result<T> types here
  pub fn new(configuration: WindowConfiguration, max_fps: u32) -> Result<Self> {
    let sdl_context = sdl2::init()?;
    let audio_subsystem = sdl_context.audio()?;
    let video_subsystem = sdl_context.video()?;
    let timer_subsystem = sdl_context.timer()?;

    let window = video_subsystem
        .window(configuration.title, configuration.width, configuration.height)
        .position_centered()
        .resizable()
        .allow_highdpi()
        .build()
        .unwrap();

    let window_canvas = window.into_canvas().build().unwrap();
    let event_pump = sdl_context.event_pump().unwrap();

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
      window_canvas,
      event_pump,
      keyboard_state,
      mouse_state,
      max_frame_time: 1000 / max_fps,
      is_closing: false,
      delta_clock: Clock::new(),
    })
  }
}

impl DesktopHost {
  /// Sets the title of the window.
  pub fn set_title(&mut self, title: &String) {
    self.window_canvas.window_mut().set_title(title.as_str()).unwrap();
  }
}

impl Host for DesktopHost {
  fn width(&self) -> u32 { self.window_canvas.window().size().0 }
  fn height(&self) -> u32 { self.window_canvas.window().size().1 }
  fn is_closing(&self) -> bool { self.is_closing }

  fn tick<C>(&mut self, mut callback: C)
    where C: FnMut(&mut Self, f64) -> () {
    // pump window events for the SDL2 window
    for event in self.event_pump.poll_iter() {
      use sdl2::event::Event;

      match event {
        Event::KeyDown { keycode: Some(key), .. } => {
          match key {
            Keycode::Escape => {
              self.is_closing = true;
            }
            _ => {}
          }
        }
        Event::Quit { .. } => {
          self.is_closing = true;
        }
        _ => {}
      }
    }

    // update the input device state
    self.mouse_state = self.event_pump.mouse_state();
    self.keyboard_state = self.event_pump.keyboard_state()
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

    // don't eat the CPU; cap the FPS
    let frame_end = self.timer_subsystem.ticks();
    let frame_time = frame_end - frame_start;

    if frame_time < self.max_frame_time {
      self.timer_subsystem.delay(self.max_frame_time - frame_time);
    }
  }

  fn exit(&mut self) {
    self.is_closing = true;
  }
}

impl AudioDevice for DesktopHost {
  fn play<A>(&mut self, _audio_clip: &AudioClip) {
    unimplemented!()
  }
}

impl GraphicsDevice for DesktopHost {
  fn clear(&mut self, color: Color) {
    self.window_canvas.set_draw_color((color.r, color.g, color.b));
    self.window_canvas.clear()
  }

  fn present(&mut self) {
    self.window_canvas.present();
  }
}

impl InputDevice for DesktopHost {
  fn is_pressed(&self, binding: impl Into<Keycode>) -> bool {
    self.keyboard_state.contains(&binding.into())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_create_a_desktop_platform() {
    let platform = DesktopPlatform {
      configuration: WindowConfiguration {
        title: "Platform Test",
        width: 1920,
        height: 1080,
        show_cursor: true,
      },
      max_fps: 30,
    };
    let mut host = platform.build().unwrap();

    host.tick(|_host, _delta_time| {});
  }
}