use surreal::engine::{Configuration, Engine};
use surreal::graphics::Color;
use surreal::ui::UserInterface;
use surreal_editor::EditorWindow;

/// Entry point for the Surreal editor application.
fn main() {
  let configuration = Configuration {
    title: "Surreal Editor",
    size: (1280, 1024),
    vsync_enabled: true,
    update_continuously: false,
    run_in_background: false,
    transparent_window: true,
    ..Default::default()
  };

  Engine::start(configuration, |engine, _| {
    let mut interface = UserInterface::new(&engine.graphics);
    let mut editor_window = EditorWindow::default();

    engine.run_variable_step(|engine, _| {
      engine.graphics.clear_color_buffer(Color::rgba(0.2, 0.2, 0.2, 0.8));

      interface.run(engine, |egui| {
        editor_window.show(egui);
      });
    });
  });
}
