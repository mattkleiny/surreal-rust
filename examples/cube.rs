//! A spinning cube, very exciting.

use surreal::{prelude::*, prototype::*};

fn main() {
  EngineBuilder::default()
    .with_title("Spinning Cube")
    .start(|engine, _| {
      let graphics = &engine.graphics;

      let mesh: Mesh<Vertex2> = Mesh::from_brush(graphics, &Cube::default());
      let material = load_built_in_material(graphics, BuiltInShader::Wire);

      engine.run_variable_step(|engine, _| {
        engine.graphics.clear_color_buffer(Color::rgb(0.1, 0.1, 0.1));

        mesh.draw(&material, PrimitiveTopology::Triangles);

        if let Some(keyboard) = &engine.input.keyboard {
          if keyboard.is_key_pressed(Key::Escape) {
            engine.quit();
          }
        }
      })
    })
    .expect("An unexpected error occurred");
}
