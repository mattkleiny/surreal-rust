//! User interface support.
//!
//! Internally we integrate the excellent egui library into the core engine.

use std::collections::HashMap;

use crate::graphics::*;
use crate::maths::{Matrix4x4, Rectangle};

/// A shader program to use for egui UI rendering.
const SHADER_UI_STANDARD: &'static str = include_str!("../assets/shaders/ui-standard.glsl");

/// A provider for [`egui::RawInput`] .
pub trait RawInputProvider {
  /// Retrieves raw input for this frame.
  fn get_raw_input(&self) -> &egui::RawInput;
}

/// Describes how to set-up a `UserInterfaceContext` for egui.
pub struct UserInterfaceContextDescriptor {
  pub projection_view: Matrix4x4<f32>
}

impl RenderContextDescriptor for UserInterfaceContextDescriptor {
  type Context = UserInterfaceContext;

  fn create(&self, server: &GraphicsServer) -> Self::Context {
    let shader = ShaderProgram::from_string(&server, SHADER_UI_STANDARD).unwrap();
    let mut material = Material::new(server, &shader);

    material.set_uniform("u_projectionView", &self.projection_view);

    Self::Context {
      graphics: server.clone(),
      context: egui::Context::default(),
      material,
      mesh: Mesh::new(server, BufferUsage::Dynamic),
      textures: HashMap::new(),
    }
  }
}

/// A context for immediate mode user interface rendering via `egui`.
pub struct UserInterfaceContext {
  graphics: GraphicsServer,
  context: egui::Context,
  material: Material,
  mesh: Mesh<Vertex2>,
  textures: HashMap<egui::TextureId, Texture>,
}

impl UserInterfaceContext {
  pub fn run(&mut self, input: &impl RawInputProvider, body: impl FnMut(&egui::Context)) {
    let raw_input = input.get_raw_input().clone();
    let full_output = self.context.run(raw_input, body);

    // apply textures delta
    for (id, image_delta) in full_output.textures_delta.set {
      // convert image representations to our color format
      // collect desired image width and height
      let (pixels, [width, height]) = match image_delta.image {
        egui::ImageData::Color(image) => {
          let pixels = image.pixels
            .iter()
            .map(|pixel| Color32::rgba(
              pixel.r(),
              pixel.g(),
              pixel.b(),
              pixel.a(),
            ))
            .collect::<Vec<_>>();

          (pixels, image.size)
        }
        egui::ImageData::Font(image) => {
          // TODO: gamma correction?
          let pixels = image.pixels
            .iter()
            .map(|pixel| Color32::rgba(
              (*pixel * 255.0) as u8,
              (*pixel * 255.0) as u8,
              (*pixel * 255.0) as u8,
              (*pixel * 255.0) as u8,
            ))
            .collect::<Vec<_>>();

          (pixels, image.size)
        }
      };

      match image_delta.pos {
        None => {
          // create new texture
          let mut texture = Texture::new(&self.graphics);

          texture.write_pixels(width, height, &pixels);

          self.textures.insert(id, texture);
        }
        Some([x, y]) => {
          // update existing texture
          let texture = self.textures.get_mut(&id).expect("Texture not found");
          let region = Rectangle::from_corner_points(x, y, x + width, y + height);

          texture.write_sub_pixels(&region, &pixels);
        }
      }
    }

    // free textures that are no longer in use
    for id in full_output.textures_delta.free {
      self.textures.remove(&id);
    }

    // create meshes from shapes
    for clipped_primitive in self.context.tessellate(full_output.shapes) {
      match clipped_primitive.primitive {
        egui::epaint::Primitive::Mesh(mesh) => {
          let _clip_rect = clipped_primitive.clip_rect;
          let vertices = mesh.vertices;
          let indices = mesh.indices;
          let _texture_id = mesh.texture_id;

          // update our single mesh shape and re-render it
          self.mesh.with_buffers(|vertex_buffer, index_buffer| {
            let vertices = unsafe {
              std::slice::from_raw_parts(vertices.as_ptr() as *const Vertex2, vertices.len())
            };

            vertex_buffer.write_data(&vertices);
            index_buffer.write_data(&indices);
          });

          // TODO: set up blending state for material
          // TODO: set up texture and screen size uniforms
          // TODO: set up back-face culling on material
          // TODO: bind clip rect as uniforms

          // render mesh using material
          self.mesh.draw(&self.material, PrimitiveTopology::Triangles);
        }
        egui::epaint::Primitive::Callback(_) => panic!("Custom rendering callbacks not yet supported"),
      }
    }

    // TODO: apply platform output somehow?
    let _platform_output = full_output.platform_output;
    let _needs_repaint = full_output.needs_repaint;
  }
}

impl RenderContext for UserInterfaceContext {}