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
    let mut renderer = create_forward_pipeline(
      &engine.graphics,
      &ForwardConfiguration {
        clear_color: Some(Color::rgba(0.2, 0.2, 0.2, 0.8)),
        render_resolution: Some((256, 144)),
      },
    );

    // set-up scene
    let mut scene = Scene::default();

    scene.camera = Camera {
      position: vec2(0., 0.),
      ..Default::default()
    };

    scene.entities.add(Entity {
      position: vec2(0., 0.),
      sprite: sprite.clone(),
    });

    engine.run_variable_step(|engine, tick| {
      scene.update(tick.time.delta_time);
      renderer.render(&scene, &scene.camera, &tick.time);

      if let Some(keyboard) = &engine.input.keyboard {
        if keyboard.is_key_pressed(Key::Escape) {
          tick.exit();
        }
      }
    });
  });
}

#[derive(Default)]
struct Scene {
  pub camera: Camera,
  pub entities: Arena<Entity>,
}

struct Camera {
  pub position: Vector2<f32>,
  pub width: f32,
  pub height: f32,
  pub near: f32,
  pub far: f32,
}

struct Entity {
  pub position: Vector2<f32>,
  pub sprite: Handle<Texture>,
}

impl Default for Camera {
  fn default() -> Self {
    Self {
      position: vec2(0., 0.),
      width: 256.,
      height: 144.,
      near: 0.,
      far: 100.,
    }
  }
}

impl Scene {
  pub fn update(&mut self, delta_time: f32) {
    for (_, entity) in self.entities.iter_mut() {
      entity.update(delta_time);
    }
  }
}

impl Entity {
  pub fn update(&mut self, delta_time: f32) {
    self.position += vec2(1., 1.) * delta_time * 20.;
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
    )
  }
}

impl RenderScene for Scene {
  fn cull_visible_objects(&self, frustum: &CameraFrustum, results: &mut Vec<CullingResult>) {
    let camera_pos = vec2(frustum.position.x, frustum.position.y);

    for (index, entity) in &self.entities {
      let distance = (camera_pos - entity.position).length_squared();

      results.push(CullingResult {
        id: index.into(),
        distance_metric: distance,
        material_key: MaterialKey {
          flags: MaterialFlags::TRANSPARENT,
        },
      })
    }
  }

  fn render(&self, id: u64, manager: &mut RenderContextManager) {
    let index = ArenaIndex::from(id);

    if let Some(entity) = self.entities.get(index) {
      manager.render(entity);
    }
  }
}

impl RenderCamera for Camera {
  fn compute_frustum(&self) -> CameraFrustum {
    CameraFrustum {
      position: vec3(self.position.x, self.position.y, 10.),
      ..Default::default()
    }
  }

  fn projection_view(&self) -> Matrix4x4 {
    Matrix4x4::orthographic(self.width, self.height, self.near, self.far)
  }
}
