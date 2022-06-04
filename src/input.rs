//! A lightweight and cross-platform input engine.

use egui::RawInput;

pub use keyboard::*;
pub use mouse::*;
use winit::event::{ElementState, ModifiersState, MouseScrollDelta};

use crate::maths::Vector2;

mod keyboard;
mod mouse;

/// The input management backend implementation for the underlying input API.
#[derive(Default)]
pub struct InputBackend {
  pub keyboard: KeyboardDevice,
  pub mouse: MouseDevice,
  raw_input: RawInput,
  pub pixels_per_point: f32,
}

impl InputBackend {
  /// Creates a new input backend.
  pub fn new() -> Self {
    Self {
      keyboard: KeyboardDevice::new(),
      mouse: MouseDevice::new(),
      raw_input: Default::default(),
      pixels_per_point: 1.0,
    }
  }

  /// Ticks the input system, apply state changes.
  pub fn tick(&mut self) {
    self.keyboard.tick();
    self.mouse.tick();

    // reset egui events
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
  pub fn on_mouse_move(&mut self, position: Vector2<f32>, window_size: Vector2<f32>) {
    self.mouse.on_mouse_moved(position, window_size);

    self
      .raw_input
      .events
      .push(egui::Event::PointerMoved(egui::Pos2 {
        x: position.x as f32 / self.pixels_per_point,
        y: position.y as f32 / self.pixels_per_point,
      }));
  }

  /// Notifies of a mouse wheel event.
  pub fn on_mouse_wheel(&mut self, delta: &MouseScrollDelta) {
    let mut delta = match delta {
      MouseScrollDelta::LineDelta(x, y) => {
        let points_per_scroll_line = 50.0;

        egui::vec2(*x, *y) * points_per_scroll_line
      }
      MouseScrollDelta::PixelDelta(delta) => {
        egui::vec2(delta.x as f32, delta.y as f32) / self.pixels_per_point
      }
    };

    delta.x *= -1.0;

    if self.raw_input.modifiers.ctrl || self.raw_input.modifiers.command {
      self
        .raw_input
        .events
        .push(egui::Event::Zoom((delta.y / 200.0).exp()));
    } else if self.raw_input.modifiers.shift {
      self
        .raw_input
        .events
        .push(egui::Event::Scroll(egui::vec2(delta.x + delta.y, 0.0)));
    } else {
      self.raw_input.events.push(egui::Event::Scroll(delta));
    }
  }

  /// Notifies of a mouse button event.
  pub fn on_mouse_button(&mut self, button: MouseButton, state: ElementState) {
    self.mouse.on_mouse_button(button, state);

    let position = self.mouse.position();

    self.raw_input.events.push(egui::Event::PointerButton {
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
      modifiers: Default::default(), // TODO: implement modifiers
    })
  }

  /// Notifies of a keyboard event.
  pub fn on_keyboard_event(&mut self, event: &winit::event::KeyboardInput) {
    self.keyboard.on_keyboard_event(event);

    if let Some(virtual_key) = event.virtual_keycode {
      if let Some(key) = translate_virtual_key_code_to_egui(virtual_key) {
        self.raw_input.events.push(egui::Event::Key {
          key,
          pressed: event.state == ElementState::Pressed,
          modifiers: Default::default(), // TODO: implement modifiers
        })
      }
    }
  }
}

/// Allow this input backend to be used in egui rendering.
impl crate::ui::RawInputProvider for InputBackend {
  fn get_raw_input(&self) -> &RawInput {
    &self.raw_input
  }
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
