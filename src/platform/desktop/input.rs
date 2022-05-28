use std::collections::HashSet;

use egui::RawInput;
use winit::event::{ElementState, ModifiersState, MouseButton, MouseScrollDelta, VirtualKeyCode};

use crate::input::*;
use crate::maths::{range, Vector2};
use crate::ui::RawInputProvider;

/// The server for input management.
pub struct DesktopInput {
  keyboard: DesktopKeyboardDevice,
  mouse: DesktopMouseDevice,
  raw_input: RawInput,
  pub pixels_per_point: f32,
}

impl DesktopInput {
  /// Creates a new input server.
  pub fn new() -> Self {
    Self {
      keyboard: DesktopKeyboardDevice::new(),
      mouse: DesktopMouseDevice::new(),
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

    self.raw_input.events.push(egui::Event::PointerMoved(egui::Pos2 {
      x: position.x as f32 / self.pixels_per_point,
      y: position.y as f32 / self.pixels_per_point,
    }));
  }

  /// Notifies of a mouse wheel event.
  pub fn on_mouse_wheel(&mut self, delta: MouseScrollDelta) {
    let mut delta = match delta {
      MouseScrollDelta::LineDelta(x, y) => {
        let points_per_scroll_line = 50.0;

        egui::vec2(x, y) * points_per_scroll_line
      }
      MouseScrollDelta::PixelDelta(delta) => {
        egui::vec2(delta.x as f32, delta.y as f32) / self.pixels_per_point
      }
    };

    delta.x *= -1.0;

    if self.raw_input.modifiers.ctrl || self.raw_input.modifiers.command {
      self.raw_input.events.push(egui::Event::Zoom((delta.y / 200.0).exp()));
    } else if self.raw_input.modifiers.shift {
      self.raw_input.events.push(egui::Event::Scroll(egui::vec2(delta.x + delta.y, 0.0)));
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
        _ => egui::PointerButton::Middle
      },
      pressed: state == ElementState::Pressed,
      modifiers: Default::default(), // TODO: implement modifiers
    })
  }

  /// Notifies of a keyboard event.
  pub fn on_keyboard_event(&mut self, event: winit::event::KeyboardInput) {
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

impl InputBackend for DesktopInput {
  fn keyboard_device(&self) -> Option<&dyn KeyboardDevice> {
    Some(&self.keyboard)
  }

  fn mouse_device(&self) -> Option<&dyn MouseDevice> {
    Some(&self.mouse)
  }
}

impl RawInputProvider for DesktopInput {
  fn get_raw_input(&self) -> &RawInput {
    &self.raw_input
  }
}

/// A keyboard device for desktop platforms.
struct DesktopKeyboardDevice {
  previous_keys: HashSet<Key>,
  current_keys: HashSet<Key>,
}

impl DesktopKeyboardDevice {
  /// Creates a new keyboard device.
  pub fn new() -> Self {
    Self {
      previous_keys: HashSet::new(),
      current_keys: HashSet::new(),
    }
  }

  /// Ticks the keyboard device.
  pub fn tick(&mut self) {
    self.previous_keys.clear();
  }

  /// Handles keyboard events.
  pub fn on_keyboard_event(&mut self, event: winit::event::KeyboardInput) {
    if let Some(virtual_key_code) = event.virtual_keycode {
      if event.state == ElementState::Pressed {
        self.current_keys.insert(virtual_key_code);
        self.previous_keys.insert(virtual_key_code);
      } else {
        self.current_keys.remove(&virtual_key_code);
      }
    }
  }
}

impl KeyboardDevice for DesktopKeyboardDevice {
  fn is_key_up(&self, key: Key) -> bool {
    !self.current_keys.contains(&key)
  }

  fn is_key_down(&self, key: Key) -> bool {
    self.current_keys.contains(&key)
  }

  fn is_key_pressed(&self, key: Key) -> bool {
    self.previous_keys.contains(&key)
  }
}

/// A mouse device for desktop platforms.
struct DesktopMouseDevice {
  position: Vector2<f32>,
  normalised_position: Vector2<f32>,
  previous_buttons: HashSet<MouseButton>,
  current_buttons: HashSet<MouseButton>,
}

impl DesktopMouseDevice {
  /// Creates a new mouse device.
  pub fn new() -> Self {
    Self {
      position: Vector2::new(0., 0.),
      normalised_position: Vector2::new(0., 0.),
      previous_buttons: HashSet::new(),
      current_buttons: HashSet::new(),
    }
  }

  /// Ticks the mouse.
  pub fn tick(&mut self) {
    self.previous_buttons.clear();
  }

  /// Handles mouse movement.
  pub fn on_mouse_moved(&mut self, new_position: Vector2<f32>, window_size: Vector2<f32>) {
    self.position = new_position;
    self.normalised_position = new_position / window_size;
  }

  /// Handles mouse click events.
  pub fn on_mouse_button(&mut self, button: MouseButton, state: ElementState) {
    match state {
      ElementState::Pressed => {
        self.current_buttons.insert(button);
        self.previous_buttons.insert(button);
      }
      ElementState::Released => {
        self.current_buttons.remove(&button);
      }
    };
  }
}

impl MouseDevice for DesktopMouseDevice {
  fn position(&self) -> Vector2<f32> {
    self.position
  }

  fn normalised_position(&self) -> Vector2<f32> {
    self.normalised_position.clamp(range(0.01, 0.99))
  }

  fn is_button_up(&self, button: MouseButton) -> bool {
    !self.current_buttons.contains(&button)
  }

  fn is_button_down(&self, button: MouseButton) -> bool {
    self.current_buttons.contains(&button)
  }

  fn is_button_pressed(&self, button: MouseButton) -> bool {
    self.previous_buttons.contains(&button)
  }
}

/// Translates a virtual key code from winit to an egui key.
fn translate_virtual_key_code_to_egui(key: VirtualKeyCode) -> Option<egui::Key> {
  use egui::Key;

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
