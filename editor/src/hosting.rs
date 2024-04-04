/// The editor's configuration.
pub struct EditorConfig {
  /// The hosting mode for the editor.
  pub hosting_mode: HostingModel,
}

/// Possible hosting models for the editor.
pub enum HostingModel {
  InProcess { entry_point: fn() -> () },
  OutOfProcess { host: String, port: u16 },
}

pub fn start_editor(config: EditorConfig) {
  match config.hosting_mode {
    HostingModel::InProcess { .. } => todo!(),
    HostingModel::OutOfProcess { .. } => todo!(),
  }
}
