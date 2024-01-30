//! Command queueing for common operations against the renderer.

use std::sync::Mutex;

use super::*;

/// A thread-safe queue of [`RenderCommand`]s.
///
/// Commands form the basis of the rendering system. They are enqueued by
/// various components and systems, and then flushed to the renderer at the
/// end of the frame.
///
/// This allows for a single pass over the scene graph, and also allows for
/// the renderer to be abstracted away from the scene graph itself.
#[derive(Default)]
pub struct RenderQueue {
  commands: Mutex<Vec<RenderCommand>>,
}

#[derive(Debug)]
pub enum RenderError {
  BufferError(BufferError),
  TextureError(TextureError),
  ShaderError(ShaderError),
  MeshError(MeshError),
  TargetError(TargetError),
}

macro_rules! impl_from_error {
  ($error:tt) => {
    impl From<$error> for RenderError {
      #[inline(always)]
      fn from(error: $error) -> Self {
        Self::$error(error)
      }
    }
  };
}

impl_from_error!(BufferError);
impl_from_error!(TextureError);
impl_from_error!(ShaderError);
impl_from_error!(MeshError);
impl_from_error!(TargetError);

/// A single command for a [`RenderQueue`] to execute.
#[allow(dead_code)]
enum RenderCommand {
  /// Sets the render target to the given target.
  SetRenderTarget { target_id: TargetId },
  /// Sets the render target to the display.
  SetRenderTargetToDisplay,
  /// Clears the active color buffer to the given color.
  ClearColorBuffer { color: Color },
  /// Clears the active depth buffer.
  ClearDepthBuffer { depth: f32 },
  /// Enables the given shader and render state.
  SetShader {
    shader_id: ShaderId,
    uniforms: Box<ShaderUniformSet>,
    blend_state: BlendState,
    culling_mode: CullingMode,
    scissor_mode: ScissorMode,
  },
  /// Sets the given uniform on the given shader by it's name.
  SetUniformByKey {
    shader_id: ShaderId,
    key: String,
    uniform: ShaderUniform,
  },
  /// Sets the given uniform on the given shader by it's location.
  SetUniformByLocation {
    shader_id: ShaderId,
    location: usize,
    uniform: ShaderUniform,
  },
  /// Issues a draw call for the given mesh.
  DrawMesh {
    mesh_id: MeshId,
    topology: PrimitiveTopology,
    vertex_count: usize,
    index_count: usize,
  },
}

impl RenderQueue {
  /// Creates a new [`RenderQueue`].
  pub fn new() -> Self {
    Self::default()
  }

  /// Sets the render target to the given target.
  pub fn set_render_target(&mut self, target_id: TargetId) {
    self.enqueue(RenderCommand::SetRenderTarget { target_id });
  }

  /// Sets the render target to the display.
  pub fn set_render_target_to_display(&mut self) {
    self.enqueue(RenderCommand::SetRenderTargetToDisplay);
  }

  /// Clears the active color buffer to the given color.
  pub fn clear_color_buffer(&mut self, color: Color) {
    self.enqueue(RenderCommand::ClearColorBuffer { color });
  }

  /// Clears the active depth buffer.
  pub fn clear_depth_buffer(&mut self, depth: f32) {
    self.enqueue(RenderCommand::ClearDepthBuffer { depth });
  }

  /// Enables the given shader and render state.
  pub fn set_shader(
    &mut self,
    shader_id: ShaderId,
    uniforms: Box<ShaderUniformSet>,
    blend_state: BlendState,
    culling_mode: CullingMode,
    scissor_mode: ScissorMode,
  ) {
    self.enqueue(RenderCommand::SetShader {
      shader_id,
      uniforms,
      blend_state,
      culling_mode,
      scissor_mode,
    });
  }

  /// Enables the given material.
  pub fn set_material(&mut self, material: &Material) {
    self.set_shader(
      material.shader().id(),
      Box::new(material.uniforms().clone()),
      material.blend_state(),
      material.culling_mode(),
      material.scissor_mode(),
    );
  }

  /// Sets the given uniform on the given shader by it's name.
  pub fn set_uniform_by_key(&mut self, shader_id: ShaderId, key: String, uniform: ShaderUniform) {
    self.enqueue(RenderCommand::SetUniformByKey {
      shader_id,
      key,
      uniform,
    });
  }

  /// Sets the given uniform on the given shader by it's location.
  pub fn set_uniform_by_location(&mut self, shader_id: ShaderId, location: usize, uniform: ShaderUniform) {
    self.enqueue(RenderCommand::SetUniformByLocation {
      shader_id,
      location,
      uniform,
    });
  }

  /// Issues a draw call for the given mesh.
  pub fn draw_mesh(&mut self, mesh_id: MeshId, topology: PrimitiveTopology, vertex_count: usize, index_count: usize) {
    self.enqueue(RenderCommand::DrawMesh {
      mesh_id,
      topology,
      vertex_count,
      index_count,
    });
  }

  /// Clears all [`RenderCommand`] from the queue.
  pub fn clear(&mut self) {
    let mut commands = self.commands.lock().unwrap();

    commands.clear();
  }

  /// Flushes all [`RenderCommand`]s in the queue to the given renderer.
  pub fn flush(&mut self, graphics: &GraphicsEngine) -> Result<(), RenderError> {
    let mut commands = self.commands.lock().unwrap();

    for command in commands.drain(..) {
      match command {
        RenderCommand::SetRenderTarget { target_id } => {
          graphics.target_activate(target_id)?;
        }
        RenderCommand::SetRenderTargetToDisplay => {
          graphics.target_set_default()?;
        }
        RenderCommand::ClearColorBuffer { color } => {
          graphics.clear_color_buffer(color);
        }
        RenderCommand::ClearDepthBuffer { depth } => {
          graphics.clear_depth_buffer(depth);
        }
        RenderCommand::SetShader {
          shader_id,
          uniforms,
          blend_state,
          culling_mode,
          scissor_mode,
        } => {
          graphics.set_blend_state(blend_state);
          graphics.set_culling_mode(culling_mode);
          graphics.set_scissor_mode(scissor_mode);

          for (key, uniform) in uniforms.iter() {
            let location = graphics
              .shader_uniform_location(shader_id, key)
              .ok_or(ShaderError::InvalidUniform)?;

            graphics.shader_set_uniform(shader_id, location, uniform)?;
          }

          graphics.shader_activate(shader_id)?;
        }
        RenderCommand::SetUniformByKey {
          shader_id,
          key,
          uniform,
        } => {
          let location = graphics
            .shader_uniform_location(shader_id, &key)
            .ok_or(ShaderError::InvalidUniform)?;

          graphics.shader_set_uniform(shader_id, location, &uniform)?;
        }
        RenderCommand::SetUniformByLocation {
          shader_id,
          location,
          uniform,
        } => {
          graphics.shader_set_uniform(shader_id, location, &uniform)?;
        }
        RenderCommand::DrawMesh {
          mesh_id,
          topology,
          vertex_count,
          index_count,
        } => {
          graphics.mesh_draw(mesh_id, topology, vertex_count, index_count)?;
        }
      }
    }

    Ok(())
  }

  /// Enqueues a new [`RenderCommand`].
  fn enqueue(&mut self, command: RenderCommand) {
    let mut commands = self.commands.lock().unwrap();

    commands.push(command);
  }
}

/// Allow [`RenderCommandQueue`] to be used as a [`RenderContext`].
impl RenderContext for RenderQueue {
  fn on_begin_frame(&mut self, _graphics: &GraphicsEngine) {
    self.clear();
  }

  fn on_end_frame(&mut self, graphics: &GraphicsEngine) {
    self.flush(graphics).expect("Failed to flush render queue");
  }
}
