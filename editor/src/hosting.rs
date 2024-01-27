/// Possible hosting modes for the editor.
pub enum HostingMode {
  InProcess,
  OutOfProcess,
}

/// The editor's configuration.
pub struct EditorConfig {
  /// The hosting mode for the editor.
  pub hosting_mode: HostingMode,
}

pub fn start_editor(config: EditorConfig) {
  match config.hosting_mode {
    HostingMode::InProcess => todo!(),
    HostingMode::OutOfProcess => todo!(),
  }
}
