//! A lightweight and cross-platform input engine.

use egui::RawInput;
use winit::event::{ElementState, KeyboardInput, ModifiersState, MouseScrollDelta};

pub use keyboard::*;
pub use mouse::*;

use crate::maths::{vec2, Vec2};

mod keyboard;
mod mouse;

/// The input management backend implementation for the underlying input API.
pub struct InputServer {
  // devices
  pub keyboard: Option<KeyboardDevice>,
  pub mouse: Option<MouseDevice>,

  // egui state
  pub pixels_per_point: f32,
  pub raw_input: RawInput,
  pub exclusive_keyboard_input: bool,
  pub exclusive_pointer_input: bool,
  actual_mouse_pos: Vec2,
}

impl InputServer {
  /// Creates a new input backend.
  pub fn new(pixels_per_point: f32) -> Self {
    Self {
      keyboard: Some(KeyboardDevice::default()),
      mouse: Some(MouseDevice::default()),
      pixels_per_point,

      raw_input: RawInput::default(),
      actual_mouse_pos: vec2(0., 0.),
      exclusive_keyboard_input: false,
      exclusive_pointer_input: false,
    }
  }

  /// Ticks the input system, apply state changes.
  pub fn tick(&mut self) {
    if let Some(keyboard) = &mut self.keyboard {
      if !self.exclusive_keyboard_input {
        keyboard.tick();
      }
    }

    if let Some(mouse) = &mut self.mouse {
      if !self.exclusive_pointer_input {
        mouse.tick();
      }
    }

    self.raw_input.events.clear();
  }

  /// Notifies of a change to keyboard modifiers.
  pub fn on_modifiers_changed(&mut self, modifiers: ModifiersState) {
    self.raw_input.modifiers.alt = modifiers.alt();
    self.raw_input.modifiers.ctrl = modifiers.ctrl();
    self.raw_input.modifiers.shift = modifiers.shift();
    self.raw_input.modifiers.command = modifiers.ctrl();
  }

  /// Notifies of a mouse movement event.
  pub fn on_mouse_move(&mut self, position: Vec2, window_size: Vec2) {
    if let Some(mouse) = &mut self.mouse {
      let event = egui::Event::PointerMoved(egui::Pos2 {
        x: position.x as f32 / self.pixels_per_point,
        y: position.y as f32 / self.pixels_per_point,
      });

      self.raw_input.events.push(event);
      self.actual_mouse_pos = position;

      if !self.exclusive_pointer_input {
        mouse.on_mouse_moved(position, window_size);
      }
    }
  }

  /// Notifies of a mouse wheel event.
  pub fn on_mouse_wheel(&mut self, delta: &MouseScrollDelta) {
    let mut delta = match delta {
      MouseScrollDelta::LineDelta(x, y) => {
        let points_per_scroll_line = 50.0;

        egui::vec2(*x, *y) * points_per_scroll_line
      }
      MouseScrollDelta::PixelDelta(delta) => egui::vec2(delta.x as f32, delta.y as f32) / self.pixels_per_point,
    };

    delta.x *= -1.0;

    if self.raw_input.modifiers.ctrl || self.raw_input.modifiers.command {
      let event = egui::Event::Zoom((delta.y / 200.0).exp());

      self.raw_input.events.push(event);
    } else if self.raw_input.modifiers.shift {
      let event = egui::Event::Scroll(egui::vec2(delta.x + delta.y, 0.0));

      self.raw_input.events.push(event);
    } else {
      let event = egui::Event::Scroll(delta);

      self.raw_input.events.push(event);
    }
  }

  /// Notifies of a mouse button event.
  pub fn on_mouse_button(&mut self, button: MouseButton, state: ElementState) {
    if let Some(mouse) = &mut self.mouse {
      let position = self.actual_mouse_pos;
      let event = egui::Event::PointerButton {
        pos: egui::Pos2 {
          x: position.x as f32 / self.pixels_per_point,
          y: position.y as f32 / self.pixels_per_point,
        },
        button: match button {
          MouseButton::Left => egui::PointerButton::Primary,
          MouseButton::Right => egui::PointerButton::Secondary,
          _ => egui::PointerButton::Middle,
        },
        pressed: state == ElementState::Pressed,
        modifiers: self.raw_input.modifiers,
      };

      self.raw_input.events.push(event);

      if !self.exclusive_pointer_input {
        mouse.on_mouse_button(button, state);
      }
    }
  }

  /// Notifies of a keyboard event.
  pub fn on_keyboard_event(&mut self, event: &KeyboardInput) {
    if let Some(keyboard) = &mut self.keyboard {
      if let Some(virtual_key) = event.virtual_keycode {
        if let Some(key) = translate_virtual_key_code_to_egui(virtual_key) {
          let event = egui::Event::Key {
            key,
            pressed: event.state == ElementState::Pressed,
            modifiers: self.raw_input.modifiers,
          };

          self.raw_input.events.push(event)
        }
      }

      if !self.exclusive_keyboard_input {
        keyboard.on_keyboard_event(event);
      }
    }
  }

  /// Notifies of a character event.
  pub fn on_character_received(&mut self, character: char) {
    if is_printable_char(character) {
      let event = egui::Event::Text(character.to_string());

      self.raw_input.events.push(event);
    }
  }
}

/// Winit sends special keys (backspace, delete, F1, …) as characters.
/// Ignore those.
/// We also ignore '\r', '\n', '\t'.
/// Newlines are handled by the `Key::Enter` event.
fn is_printable_char(chr: char) -> bool {
  let is_in_private_use_area =
    '\u{e000}' <= chr && chr <= '\u{f8ff}' || '\u{f0000}' <= chr && chr <= '\u{ffffd}' || '\u{100000}' <= chr && chr <= '\u{10fffd}';

  !is_in_private_use_area && !chr.is_ascii_control()
}

/// Translates a virtual key code from winit to an egui key.
fn translate_virtual_key_code_to_egui(key: Key) -> Option<egui::Key> {
  use egui::Key;
  use winit::event::VirtualKeyCode;

  Some(match key {
    VirtualKeyCode::Down => Key::ArrowDown,
    VirtualKeyCode::Left => Key::ArrowLeft,
    VirtualKeyCode::Right => Key::ArrowRight,
    VirtualKeyCode::Up => Key::ArrowUp,

    VirtualKeyCode::Escape => Key::Escape,
    VirtualKeyCode::Tab => Key::Tab,
    VirtualKeyCode::Back => Key::Backspace,
    VirtualKeyCode::Return => Key::Enter,
    VirtualKeyCode::Space => Key::Space,

    VirtualKeyCode::Insert => Key::Insert,
    VirtualKeyCode::Delete => Key::Delete,
    VirtualKeyCode::Home => Key::Home,
    VirtualKeyCode::End => Key::End,
    VirtualKeyCode::PageUp => Key::PageUp,
    VirtualKeyCode::PageDown => Key::PageDown,

    VirtualKeyCode::Key0 | VirtualKeyCode::Numpad0 => Key::Num0,
    VirtualKeyCode::Key1 | VirtualKeyCode::Numpad1 => Key::Num1,
    VirtualKeyCode::Key2 | VirtualKeyCode::Numpad2 => Key::Num2,
    VirtualKeyCode::Key3 | VirtualKeyCode::Numpad3 => Key::Num3,
    VirtualKeyCode::Key4 | VirtualKeyCode::Numpad4 => Key::Num4,
    VirtualKeyCode::Key5 | VirtualKeyCode::Numpad5 => Key::Num5,
    VirtualKeyCode::Key6 | VirtualKeyCode::Numpad6 => Key::Num6,
    VirtualKeyCode::Key7 | VirtualKeyCode::Numpad7 => Key::Num7,
    VirtualKeyCode::Key8 | VirtualKeyCode::Numpad8 => Key::Num8,
    VirtualKeyCode::Key9 | VirtualKeyCode::Numpad9 => Key::Num9,

    VirtualKeyCode::A => Key::A,
    VirtualKeyCode::B => Key::B,
    VirtualKeyCode::C => Key::C,
    VirtualKeyCode::D => Key::D,
    VirtualKeyCode::E => Key::E,
    VirtualKeyCode::F => Key::F,
    VirtualKeyCode::G => Key::G,
    VirtualKeyCode::H => Key::H,
    VirtualKeyCode::I => Key::I,
    VirtualKeyCode::J => Key::J,
    VirtualKeyCode::K => Key::K,
    VirtualKeyCode::L => Key::L,
    VirtualKeyCode::M => Key::M,
    VirtualKeyCode::N => Key::N,
    VirtualKeyCode::O => Key::O,
    VirtualKeyCode::P => Key::P,
    VirtualKeyCode::Q => Key::Q,
    VirtualKeyCode::R => Key::R,
    VirtualKeyCode::S => Key::S,
    VirtualKeyCode::T => Key::T,
    VirtualKeyCode::U => Key::U,
    VirtualKeyCode::V => Key::V,
    VirtualKeyCode::W => Key::W,
    VirtualKeyCode::X => Key::X,
    VirtualKeyCode::Y => Key::Y,
    VirtualKeyCode::Z => Key::Z,

    _ => return None,
  })
}
