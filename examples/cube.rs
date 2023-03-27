//! A spinning cube, very exciting.

use surreal::{prelude::*, prototype::*};

fn main() {
  EngineBuilder::default()
    .with_title("Spinning Cube")
    .start(|engine, _| {
      let graphics = &engine.graphics;

      let mesh: Mesh<Vertex2> = Mesh::from_brush(graphics, &Cube::default());
      let mut material = load_built_in_material(graphics, BuiltInShader::Wire);

      let projection_view = Mat4::perspective_rh_gl(1.0, 1.0, 0.1, 100.0);

      material.set_uniform("u_projectionView", projection_view);

      engine.run_variable_step(|engine, _| {
        engine
          .graphics
          .clear_color_buffer(Color::rgb(0.1, 0.1, 0.1));

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
