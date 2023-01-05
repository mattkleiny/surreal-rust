//! A spinning cube, very exciting.

use surreal::prelude::*;
use surreal::prototype::*;

fn main() {
  let configuration = Configuration {
    title: "Spinning Cube",
    transparent_window: true,
    ..Default::default()
  };

  Engine::start(configuration, |engine, _| {
    let graphics = &engine.graphics;

    let mesh: Mesh<Vertex2> = Mesh::from_brush(graphics, &Cube::default());
    let material = load_built_in_material(graphics, BuiltInShader::Wire);

    engine.run_variable_step(|engine, tick| {
      engine.graphics.clear_color_buffer(Color::rgba(0.2, 0.2, 0.2, 0.8));

      mesh.draw(&material, PrimitiveTopology::Triangles);

      if let Some(keyboard) = &engine.input.keyboard {
        if keyboard.is_key_pressed(Key::Escape) {
          tick.exit();
        }
      }
    })
  });
}
