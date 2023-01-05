use surreal::assets::AssetManager;
use surreal::engine::{Configuration, Engine};
use surreal::graphics::{Color, RenderContextManager};
use surreal::input::Key;
use surreal::maths::Mat4;
use surreal::scene::{SceneEvent, SceneGraph};

const WIDTH: f32 = 1920.;
const HEIGHT: f32 = 1080.;

/// Bootstraps an example for the module
pub fn bootstrap(name: &'static str, factory: impl Fn(&Engine, &AssetManager) -> SceneGraph) {
  let configuration = Configuration {
    title: name,
    size: (WIDTH as u32, HEIGHT as u32),
    transparent_window: true,
    ..Default::default()
  };

  Engine::start(configuration, |engine, assets| {
    let mut scene = factory(&engine, &assets);
    let mut renderer = RenderContextManager::new(&engine.graphics);

    renderer.add_descriptor(surreal_scene2d::rendering::sprites::SpriteContextDescriptor {
      projection_view: Mat4::orthographic_rh_gl(-WIDTH / 2., WIDTH / 2., HEIGHT / 2., -HEIGHT / 2., 0., 100.),
      ..Default::default()
    });

    engine.run_variable_step(|engine, tick| {
      engine.graphics.clear_color_buffer(Color::rgba(0.2, 0.2, 0.2, 0.8));

      renderer.begin_frame();

      scene.notify(SceneEvent::Update(tick.time.delta_time));
      scene.notify(SceneEvent::Render(&mut renderer));

      renderer.end_frame();

      if let Some(keyboard) = &engine.input.keyboard {
        if keyboard.is_key_pressed(Key::Escape) {
          tick.exit();
        }
      }
    });
  });
}
