//! Windowing implementation for the editor.

/// Hosts the editor windows, processing events and rendering.
pub struct EditorWindowHost {
  windows: Vec<Box<dyn EditorWindow>>,
}

impl EditorWindowHost {
  /// Creates a new editor window host.
  pub fn new() -> Self {
    EditorWindowHost { windows: Vec::new() }
  }

  /// Adds a window to the editor.
  pub fn add_window<T: EditorWindow + 'static>(&mut self, window: T) {
    self.windows.push(Box::new(window));
  }

  /// Updates the windows, processing all window events.
  pub fn update(&mut self) -> bool {
    let mut running_count = 0;
    let mut closed_windows = Vec::new();

    // update all windows
    for (index, window) in &mut self.windows.iter_mut().enumerate() {
      if window.update() {
        running_count += 1;
      } else {
        closed_windows.push(index);
      }
    }

    // remove all closed windows
    for index in closed_windows {
      self.windows.remove(index);
    }

    running_count > 0
  }

  /// Presents the windows to the screen.
  pub fn present(&mut self) {
    for window in &mut self.windows {
      window.present();
    }
  }
}

/// A window that can be hosted by the editor.
pub trait EditorWindow {
  /// Updates the window, processing all window events.
  fn update(&mut self) -> bool;

  /// Presents the window to the screen.
  fn present(&mut self);
}

/// Settings for an editor window.
pub struct EditorWindowSettings {}

/// A panel that can be hosted within an editor window.
pub trait EditorPanel {}

/// Layout settings for an editor panel.
pub struct EditorPanelLayout {}

pub struct ProjectWindow {}

impl ProjectWindow {
  pub fn new() -> Self {
    ProjectWindow {}
  }
}

impl EditorWindow for ProjectWindow {
  fn update(&mut self) -> bool {
    true
  }

  fn present(&mut self) {}
}
