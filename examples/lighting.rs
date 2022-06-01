#![windows_subsystem = "windows"]

use surreal::prelude::*;

fn main() {
  let platform = DesktopPlatform::new(Configuration {
    title: "Lighting Test",
    ..Default::default()
  });

  Game::start(platform, |mut game, assets| {
    let graphics = &game.host.graphics;
    
    // TODO: emissive map from a single pass of all materials?
    // TODO: combine emissive map with a bloom sample?

    let material: &Material = assets.load_asset("assets/shaders/sprite-lit.glsl").expect("Failed to load shader");

    let mesh = Mesh::create(graphics, |tessellator| {
      tessellator.add_triangle(&[
        LitVertex {
          position: vec2(-0.5, -0.5),
          color: Color32::RED,
          emission: 0.2,
        },
        LitVertex {
          position: vec2(0.0, 0.5),
          color: Color32::GREEN,
          emission: 0.2,
        },
        LitVertex {
          position: vec2(0.5, -0.5),
          color: Color32::BLUE,
          emission: 0.2,
        },
      ])
    });

    game.run_variable_step(|game| {
      game.host.graphics.clear_color_buffer(Color::BLACK);
      mesh.draw(&material, PrimitiveTopology::Triangles);

      if let Some(keyboard) = game.host.input.keyboard_device() {
        if keyboard.is_key_pressed(Key::Escape) {
          game.exit();
        }
      }
    });
  });
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct LitVertex {
  pub position: Vector2<f32>,
  pub color: Color32,
  pub emission: f32,
}

impl Vertex for LitVertex {
  const DESCRIPTORS: &'static [VertexDescriptor] = &[
    VertexDescriptor {
      count: 2,
      kind: VertexKind::F32,
      should_normalize: false,
    },
    VertexDescriptor {
      count: 4,
      kind: VertexKind::U8,
      should_normalize: true,
    },
    VertexDescriptor {
      count: 1,
      kind: VertexKind::F32,
      should_normalize: false,
    },
  ];
}
