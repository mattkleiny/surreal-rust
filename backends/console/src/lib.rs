//! A backend that allows executing the engine in a standard console.
//!
//! This is useful for prototyping and experimenting, with the intention of
//! eventually moving to a more appropriate backend.

use common::FastHashSet;
use input::{KeyboardDevice, VirtualKey};

/// The main console backend.
pub struct ConsoleWindow {
  _pressed_keys: FastHashSet<VirtualKey>,
}

/// The settings for the console backend.
pub struct ConsoleSettings {
  pub title: String,
  pub width: u32,
  pub height: u32,
  pub rows: u32,
  pub columns: u32,
}

impl Default for ConsoleSettings {
  fn default() -> Self {
    Self {
      title: "Surreal".to_string(),
      width: 1024,
      height: 768,
      rows: 25,
      columns: 80,
    }
  }
}

impl KeyboardDevice for ConsoleWindow {
  fn is_key_down(&self, _key: VirtualKey) -> bool {
    todo!()
  }

  fn is_key_up(&self, _key: VirtualKey) -> bool {
    todo!()
  }
}

#[cfg(target_os = "windows")]
mod windows {
  //! Windows-specific console backend.
  use super::*;

  impl ConsoleWindow {
    /// Creates a new console backend.
    pub fn new(_settings: ConsoleSettings) -> ConsoleWindow {
      ConsoleWindow {
        _pressed_keys: FastHashSet::default(),
      }
    }

    /// Updates the console backend.
    pub fn update(&mut self) -> bool {
      todo!()
    }
  }
}

#[cfg(target_os = "unix")]
mod unix {
  //! Unix-specific console backend.
  use super::*;

  impl ConsoleWindow {
    /// Creates a new console backend.
    pub fn new(_settings: ConsoleSettings) -> ConsoleWindow {
      ConsoleWindow {
        _pressed_keys: FastHashSet::default(),
      }
    }

    /// Updates the console backend.
    pub fn update(&mut self) -> bool {
      todo!()
    }
  }
}

#[cfg(target_os = "macos")]
mod macos {
  //! macOS-specific console backend.
  use super::*;

  impl ConsoleWindow {
    /// Creates a new console backend.
    pub fn new(_settings: ConsoleSettings) -> ConsoleWindow {
      ConsoleWindow {
        _pressed_keys: FastHashSet::default(),
      }
    }

    /// Updates the console backend.
    pub fn update(&mut self) -> bool {
      todo!()
    }
  }
}
