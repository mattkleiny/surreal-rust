use surreal::assets::AssetManager;
use surreal::diagnostics::profiling;
use surreal::engine::{Application, Configuration, Engine, GameTick};
use surreal::ui::UserInterface;
use surreal_editor::{EditorWindow, Project};

/// Entry point for the Surreal editor application.
fn main() {
  let configuration = Configuration {
    title: "Surreal Editor",
    size: (1920, 1080),
    vsync_enabled: true,
    update_continuously: false,
    transparent_window: true,
    ..Default::default()
  };

  Engine::from_application::<EditorApplication>(configuration);
}

/// Top-level [`Application`] for the editor.
struct EditorApplication {
  user_interface: UserInterface,
  editor_window: EditorWindow,
  current_project: Option<Project>,
}

impl Application for EditorApplication {
  fn new(engine: &Engine, _assets: &AssetManager) -> surreal::Result<Self> {
    let user_interface = UserInterface::new(&engine.graphics);
    let editor_window = EditorWindow::default();

    let project = Project::open(
      std::env::current_dir()?
        .to_str()
        .ok_or(surreal::anyhow!("Failed to determine current directory"))?,
    )?;

    Ok(Self {
      user_interface,
      editor_window,
      current_project: Some(project),
    })
  }

  fn on_update(&mut self, _engine: &mut Engine, _tick: &mut GameTick) {
    profiling::profile_scope!("Editor loop");
  }

  fn on_draw(&mut self, engine: &mut Engine, _tick: &mut GameTick) {
    profiling::profile_scope!("Editor draw");

    self.user_interface.run(engine, |egui| {
      self.editor_window.show(egui);
    });
  }
}
