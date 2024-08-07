//! The main editor binary for the Surreal Project.

#![allow(dead_code)]

use surreal_editor::*;

mod windows;

fn main() {
  start_editor(EditorConfig {
    hosting_mode: HostingModel::OutOfProcess {
      host: "localhost".to_string(),
      port: 8080,
    },
  })
}
