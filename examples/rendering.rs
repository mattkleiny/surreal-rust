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
      camera_position: vec2(0., 0.),
      entities: vec![Entity {
        position: vec2(0., 0.),
        sprite: sprite.clone(),
      }],
    };

    engine.run_variable_step(|engine, tick| {
      // render the scene
      pipeline.render(&scene, &scene);

      for entity in scene.entities.iter_mut() {
        entity.position += vec2(1., 1.) * tick.time.delta_time;
      }

      if engine.input.keyboard.is_key_pressed(Key::Escape) {
        tick.exit();
      }
    });
  });
}

/// A scene that can be rendered by a render pipeline.
struct Scene {
  pub camera_position: Vector2<f32>,
  pub entities: Vec<Entity>,
}

/// An entity in a scene.
struct Entity {
  pub position: Vector2<f32>,
  pub sprite: Handle<Texture>,
}

impl RenderCamera for Scene {
  fn compute_frustum(&self) -> CameraFrustum {
    CameraFrustum {
      position: vec3(self.camera_position.x, self.camera_position.y, 10.),
      ..Default::default()
    }
  }
}

impl RenderScene for Scene {
  fn cull_visible_objects(&self, frustum: &CameraFrustum, results: &mut Vec<CullingResult>) {
    for (index, entity) in self.entities.iter().enumerate() {
      let direction = vec2(frustum.position.x, frustum.position.y) - entity.position;
      let distance = direction.length_squared();

      results.push(CullingResult {
        id: index,
        distance_metric: distance,
        material_key: MaterialKey {
          flags: MaterialFlags::TRANSPARENT,
        },
      })
    }
  }

  fn render_object(&self, id: usize, manager: &mut RenderContextManager) {
    if let Some(entity) = self.entities.get(id) {
      manager.render(entity);
    }
  }
}

impl Renderable<SpriteBatchContext> for Entity {
  fn render(&self, context: &mut SpriteBatchContext) {
    context.batch.draw_sprite(
      &TextureRegion::from(&self.sprite),
      &SpriteOptions {
        position: self.position,
        rotation: 0.,
        ..Default::default()
      },
    );
  }
}
