//! The main editor binary for the Surreal Project.

use surreal_editor::{EditorWindowHost, ProjectWindow};

fn main() {
  let mut host = EditorWindowHost::default();

  host.add_window(ProjectWindow::default());

  while host.update() {
    host.present();
  }
}
