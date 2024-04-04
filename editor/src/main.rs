//! The main editor binary for the Surreal Project.

use surreal_editor::*;

fn main() {
  start_editor(EditorConfig {
    hosting_mode: HostingModel::OutOfProcess {
      host: "localhost".to_string(),
      port: 8080,
    },
  })
}
