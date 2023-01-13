use surreal::{
  assets::AssetManager,
  engine::{Engine, EngineConfig},
  graphics::Renderer,
  input::Key,
  maths::Mat4,
  scene::{SceneEvent, SceneGraph},
};

const WIDTH: f32 = 1920.;
const HEIGHT: f32 = 1080.;

/// Bootstraps an example for the module
pub fn run_example(name: &'static str, factory: impl Fn(&Engine, &AssetManager) -> SceneGraph) {
  let configuration = EngineConfig {
    title: name.to_string(),
    size: (WIDTH as u32, HEIGHT as u32),
    transparent_window: true,
    ..Default::default()
  };

  Engine::start(configuration, |engine, assets| {
    let mut scene = factory(&engine, &assets);
    let mut renderer = Renderer::new(&engine.graphics);

    renderer.add_descriptor(surreal_scene2d::SpriteContextDescriptor {
      projection_view: Mat4::orthographic_rh_gl(-WIDTH / 2., WIDTH / 2., HEIGHT / 2., -HEIGHT / 2., 0., 100.),
      ..Default::default()
    });

    engine.run_variable_step(|engine, time| {
      renderer.begin_frame();

      scene.notify(SceneEvent::Update(time.delta_time));
      scene.notify(SceneEvent::Render(&mut renderer));

      renderer.end_frame();

      if let Some(keyboard) = &engine.input.keyboard {
        if keyboard.is_key_pressed(Key::Escape) {
          engine.quit();
        }
      }
    });
  });
}
