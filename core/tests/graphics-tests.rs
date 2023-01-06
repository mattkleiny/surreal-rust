use surreal_core::graphics::{BlendFactor, BlendState, Color32, Material, Mesh, PrimitiveTopology, ShaderProgram, Texture, Vertex2};
use surreal_core::maths::{Cube, Mat4};

mod common;

/// Ensure basic [`Texture`] access doesn't crash and can read/write pixel data.
#[test]
fn texture_read_write() {
  common::bootstrap(|graphics| {
    let texture = Texture::new(graphics);

    texture.write_pixels(16, 16, &[Color32::WHITE; 16 * 16]);
    let pixels = texture.read_pixels::<Color32>();

    assert_eq!(pixels.len(), 16 * 16);
    assert_eq!(pixels, vec![Color32::WHITE; 16 * 16]);
  })
}

/// Ensure that [`ShaderProgram`]s compile, and [`Material`]s can access uniforms.
#[test]
fn shader_and_material_uniform_access() {
  common::bootstrap(|graphics| {
    let shader = ShaderProgram::from_code(
      graphics,
      r#"
        #version 330 core

        #shader_type vertex

        uniform vec4 u_color;
        uniform mat4 u_projectionView;

        out vec4 v_color;

        void main() {
          v_color = u_color;
        }

        #shader_type fragment

        in vec4 v_color;

        layout(location = 0) out vec4 color;

        void main() {
          color = v_color;
        }
      "#,
    )
    .unwrap();

    let mut material = Material::new(graphics, &shader);

    material.set_blend_state(BlendState::Enabled {
      source: BlendFactor::SrcAlpha,
      destination: BlendFactor::OneMinusSrcAlpha,
    });

    material.set_uniform("u_color", Color32::WHITE);
    material.set_uniform("u_projectionView", &Mat4::IDENTITY);

    material.bind();
  })
}

/// Ensure basic [`Mesh`]es can be constructed and can be rendered.
#[test]
fn mesh_rendering() {
  common::bootstrap(|graphics| {
    let mesh: Mesh<Vertex2> = Mesh::from_brush(graphics, &Cube::default());

    let shader = ShaderProgram::from_path(graphics, "local://../assets/shaders/canvas-standard.glsl").unwrap();
    let material = Material::new(graphics, &shader);

    mesh.draw(&material, PrimitiveTopology::Triangles);
  })
}
