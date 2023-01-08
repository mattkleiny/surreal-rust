use surreal::assets::AssetManager;
use surreal::engine::{Application, Engine, EngineConfig, GameTime};
use surreal::ui::UserInterface;
use surreal_editor::{EditorWindow, Project};

/// Entry point for the Surreal editor application.
fn main() {
  let configuration = EngineConfig {
    title: "Surreal Editor".to_string(),
    size: (1920, 1080),
    vsync_enabled: true,
    update_continuously: false,
    ..Default::default()
  };

  Engine::from_application::<EditorApplication>(configuration);
}

/// Top-level [`Application`] for the editor.
struct EditorApplication {
  user_interface: UserInterface,
  editor_window: EditorWindow,
  _current_project: Option<Project>,
}

impl Application for EditorApplication {
  fn new(engine: &mut Engine, _assets: &mut AssetManager) -> surreal::Result<Self> {
    let user_interface = UserInterface::new(&engine.graphics);
    let editor_window = EditorWindow::new();

    let project = Project::open_or_create(
      "Test Project",
      std::env::current_dir()?
        .to_str()
        .ok_or(surreal::anyhow!("Failed to determine current directory"))?,
    )?;

    engine.set_title(&format!("Surreal Editor - {} ({})", &project.details.name, &project.details.path));

    Ok(Self {
      user_interface,
      editor_window,
      _current_project: Some(project),
    })
  }

  fn on_update(&mut self, engine: &mut Engine, _time: GameTime) {
    if let Some(keyboard) = &engine.input.keyboard {
      if keyboard.is_key_pressed(surreal::input::Key::Escape) {
        engine.quit();
      }
    }
  }

  fn on_draw(&mut self, engine: &mut Engine, _time: GameTime) {
    self.user_interface.run(engine, |egui| {
      self.editor_window.show(egui);
    });
  }
}
