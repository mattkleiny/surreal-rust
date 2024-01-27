//! Command queueing for common operations against the renderer.

use std::sync::Mutex;

use common::Rectangle;

use super::*;

// TODO: implement the command queue idea more fully

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
pub enum RenderCommand {
  SetShader {
    shader_id: ShaderId,
    uniforms: Box<ShaderUniformSet>,
  },
  SetUniform {
    shader_id: ShaderId,
    location: usize,
    uniform: ShaderUniform,
  },
  SetRenderTarget {},
  DrawMesh {
    mesh_id: MeshId,
    region: Rectangle,
    tint: Color,
  },
}

impl RenderQueue {
  /// Enqueues a new [`RenderCommand`].
  pub fn enqueue(&mut self, command: RenderCommand) {
    let mut commands = self.commands.lock().unwrap();

    commands.push(command);
  }

  /// Clears all [`RenderCommand`] from the queue.
  pub fn clear(&mut self) {
    let mut commands = self.commands.lock().unwrap();

    commands.clear();
  }

  /// Flushes all [`RenderCommand`]s in the queue to the given renderer.
  pub fn flush(&mut self, _graphics: &GraphicsEngine) {
    let mut commands = self.commands.lock().unwrap();

    for _command in commands.drain(..) {
      todo!()
    }
  }
}

/// Allow [`RenderCommandQueue`] to be used as a [`RenderContext`].
impl RenderContext for RenderQueue {
  fn on_begin_frame(&mut self, _graphics: &GraphicsEngine) {
    self.clear();
  }

  fn on_end_frame(&mut self, graphics: &GraphicsEngine) {
    self.flush(graphics);
  }
}
