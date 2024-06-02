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
  /// Sets the given uniform on the given shader by its name.
  SetUniformByKey {
    shader_id: ShaderId,
    key: String,
    uniform: ShaderUniform,
  },
  /// Sets the given uniform on the given shader by its location.
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
  /// Dispatches a compute shader for execution.
  DispatchCompute {
    shader_id: ShaderId,
    group_count: (u32, u32, u32),
  },
  /// Issues a memory barrier, which can be used to synchronize memory access.
  MemoryBarrier { barrier: MemoryBarrier },
  /// Blits the given render target to the active render target.
  BlitRenderTargetToActive { target_id: TargetId, filter: TextureFilter },
}

/// Represents an error that occurred while using the render queue.
#[derive(Debug)]
pub enum RenderQueueError {
  BufferError(BufferError),
  TextureError(TextureError),
  ShaderError(ShaderError),
  MeshError(MeshError),
  TargetError(TargetError),
}

common::impl_from_error!(BufferError for RenderQueueError);
common::impl_from_error!(TextureError for RenderQueueError);
common::impl_from_error!(ShaderError for RenderQueueError);
common::impl_from_error!(MeshError for RenderQueueError);
common::impl_from_error!(TargetError for RenderQueueError);

impl RenderQueue {
  /// Creates a new [`RenderQueue`].
  pub fn new() -> Self {
    Self::default()
  }

  /// Sets the render target to the given target.
  pub fn set_render_target(&mut self, target: &RenderTarget) {
    self.enqueue(RenderCommand::SetRenderTarget { target_id: target.id() });
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

  /// Enables the given material.
  pub fn set_material(&mut self, material: &Material) {
    self.enqueue(RenderCommand::SetShader {
      shader_id: material.shader().id(),
      uniforms: Box::new(material.uniforms().clone()),
      blend_state: material.blend_state(),
      culling_mode: material.culling_mode(),
      scissor_mode: material.scissor_mode(),
    });
  }

  /// Draws the given [`Mesh`].
  pub fn draw_mesh<V: Vertex>(&mut self, mesh: &Mesh<V>, topology: PrimitiveTopology) {
    self.enqueue(RenderCommand::DrawMesh {
      mesh_id: mesh.id(),
      topology,
      vertex_count: mesh.vertices(),
      index_count: mesh.indices(),
    })
  }

  /// Blits the given [`RenderTarget`] to the display.
  pub fn blit_render_target_to_display(&mut self, target: &RenderTarget, clear_color: Option<Color>) {
    self.enqueue(RenderCommand::SetRenderTargetToDisplay);

    if let Some(color) = clear_color {
      self.enqueue(RenderCommand::ClearColorBuffer { color });
    }

    self.enqueue(RenderCommand::BlitRenderTargetToActive {
      target_id: target.id(),
      filter: TextureFilter::Linear,
    });
  }

  /// Dispatches a compute shader for execution.
  pub fn dispatch_compute(&mut self, shader: &ShaderProgram, group_count: (u32, u32, u32)) {
    self.enqueue(RenderCommand::DispatchCompute {
      shader_id: shader.id(),
      group_count,
    });
  }

  /// Issues a memory barrier, which can be used to synchronize memory access.
  pub fn memory_barrier(&mut self, barrier: MemoryBarrier) {
    self.enqueue(RenderCommand::MemoryBarrier { barrier });
  }

  /// Clears all [`RenderCommand`] from the queue.
  pub fn clear(&mut self) {
    let mut commands = self.commands.lock().unwrap();

    commands.clear();
  }

  /// Flushes all [`RenderCommand`]s in the queue to the given renderer.
  pub fn flush(&mut self) -> Result<(), RenderQueueError> {
    let mut commands = self.commands.lock().unwrap();

    for command in commands.drain(..) {
      common::profile_scope!("RenderCommand::{:?}", command.type_name());

      match command {
        RenderCommand::SetRenderTarget { target_id } => {
          graphics().target_activate(target_id)?;
        }
        RenderCommand::SetRenderTargetToDisplay => {
          graphics().target_set_default()?;
        }
        RenderCommand::ClearColorBuffer { color } => {
          graphics().clear_color_buffer(color);
        }
        RenderCommand::ClearDepthBuffer { depth } => {
          graphics().clear_depth_buffer(depth);
        }
        RenderCommand::SetShader {
          shader_id,
          uniforms,
          blend_state,
          culling_mode,
          scissor_mode,
        } => {
          graphics().set_blend_state(blend_state);
          graphics().set_culling_mode(culling_mode);
          graphics().set_scissor_mode(scissor_mode);

          for (key, uniform) in uniforms.iter() {
            let location = graphics()
              .shader_uniform_location(shader_id, key)
              .ok_or(ShaderError::InvalidUniform)?;

            graphics().shader_set_uniform(shader_id, location, uniform)?;
          }

          graphics().shader_activate(shader_id)?;
        }
        RenderCommand::SetUniformByKey {
          shader_id,
          key,
          uniform,
        } => {
          let location = graphics()
            .shader_uniform_location(shader_id, &key)
            .ok_or(ShaderError::InvalidUniform)?;

          graphics().shader_set_uniform(shader_id, location, &uniform)?;
        }
        RenderCommand::SetUniformByLocation {
          shader_id,
          location,
          uniform,
        } => {
          graphics().shader_set_uniform(shader_id, location, &uniform)?;
        }
        RenderCommand::DispatchCompute {
          shader_id,
          group_count: (x, y, z),
        } => {
          graphics().shader_dispatch_compute(shader_id, x, y, z)?;
        }
        RenderCommand::MemoryBarrier { barrier } => {
          graphics().shader_memory_barrier(barrier)?;
        }
        RenderCommand::DrawMesh {
          mesh_id,
          topology,
          vertex_count,
          index_count,
        } => {
          graphics().mesh_draw(mesh_id, topology, vertex_count, index_count)?;
        }
        RenderCommand::BlitRenderTargetToActive { target_id, filter } => {
          graphics().target_blit_to_active(target_id, None, None, filter)?;
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_basic_commands() {
    let mut queue = RenderQueue::default();

    queue.set_render_target_to_display();
    queue.clear_color_buffer(Color::BLACK);

    queue.flush().unwrap();
  }
}
