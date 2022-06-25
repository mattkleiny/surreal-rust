//! A simple rendering pipeline example.

use surreal::prelude::*;

fn main() {
  let configuration = Configuration {
    title: "Rendering Test",
    transparent_window: true,
    ..Default::default()
  };

  Engine::start(configuration, |engine, assets| {
    // load assets
    let sprite = Texture::load(&assets, "assets/sprites/bunny.png").unwrap();

    // set-up rendering
    let mut pipeline = create_forward_pipeline(
      &engine.graphics,
      &ForwardConfiguration {
        clear_color: Some(Color::rgba(0.2, 0.2, 0.2, 0.8)),
        render_resolution: Some((256, 144)),
      },
    );

    // set-up scene
    let mut scene = Scene {
      camera: Camera {
        position: vec2(0., 0.),
        ..Default::default()
      },
      ..Default::default()
    };

    engine.run_variable_step(|engine, tick| {
      pipeline.render(&scene, &scene.camera);

      if engine.input.keyboard.is_key_pressed(Key::Escape) {
        tick.exit();
      }
    });
  });
}
