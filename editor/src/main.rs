//! The main editor binary for the Surreal Project.

use surreal_editor::{EditorWindowHost, ProjectWindow};

fn main() {
  let mut host = EditorWindowHost::new();

  host.add_window(ProjectWindow::new());

  while host.update() {
    host.present();
  }
}
