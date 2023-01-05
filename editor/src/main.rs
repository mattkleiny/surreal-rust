use surreal::diagnostics::profiling;
use surreal::engine::{Configuration, Engine};
use surreal::ui::UserInterface;
use surreal_editor::EditorWindow;

// TODO: consider using the windows APIs directly to allow better integration with the platform?

/// Entry point for the Surreal editor application.
fn main() {
  let configuration = Configuration {
    title: "Surreal Editor",
    size: (1280, 1024),
    vsync_enabled: true,
    update_continuously: true,
    run_in_background: false,
    transparent_window: true,
    ..Default::default()
  };

  Engine::start(configuration, |engine, _| {
    let mut interface = UserInterface::new(&engine.graphics);
    let mut editor_window = EditorWindow::default();

    engine.run_variable_step(|engine, _| {
      profiling::profile_scope!("Editor loop");

      interface.run(engine, |egui| {
        editor_window.show(egui);
      });
    });
  });
}
