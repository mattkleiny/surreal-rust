use surreal::assets::AssetManager;
use surreal::engine::{Configuration, Engine};
use surreal::graphics::{Color, RenderContextManager};
use surreal::input::Key;
use surreal::scene::{SceneEvent, SceneGraph};

/// Bootstraps an example for the module
pub fn bootstrap(name: &'static str, factory: impl Fn(&Engine, &AssetManager) -> SceneGraph) {
  let configuration = Configuration {
    title: name,
    size: (1280, 1024),
    transparent_window: true,
    ..Default::default()
  };

  Engine::start(configuration, |engine, assets| {
    let mut renderer = RenderContextManager::new(&engine.graphics);
    let mut scene = factory(&engine, &assets);

    engine.run_variable_step(|engine, tick| {
      engine.graphics.clear_color_buffer(Color::rgba(0.2, 0.2, 0.2, 0.8));

      scene.notify(SceneEvent::Update(tick.time.delta_time));
      scene.notify(SceneEvent::Render(&mut renderer));

      if let Some(keyboard) = &engine.input.keyboard {
        if keyboard.is_key_pressed(Key::Escape) {
          tick.exit();
        }
      }
    });
  });
}
