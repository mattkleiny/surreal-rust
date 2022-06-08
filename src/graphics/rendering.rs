//! Rendering abstractions and pipelines.
//!
//! This is a series of components designed to make it simpler to build more complex render
//! pipelines than using the 'material', 'mesh', 'render targets' etc do alone.

use crate::collections::AnyMap;
use std::collections::VecDeque;

use super::*;

/// A command buffer encodes a set of instructions to be replayed against the graphics server.
///
/// Command buffers decouple the order of instructions from the executino of those instructions
/// and allow for collection of commands from across multiple threads and workers.
#[derive(Default)]
pub struct CommandBuffer {
  commands: VecDeque<Command>,
}

/// Encodes a single command in the command buffer.
enum Command {
  ClearColor(Color),
  ClearDepth,
  SetViewportSize((usize, usize)),
  SetBlendState(BlendState),
  SetScissorMode(ScissorMode),
  SetCullingMode(CullingMode),
  SetTarget(RenderTarget),
  SetTargetToDisplay,
  Blit(RenderTarget, RenderTarget, TextureFilter),
  BlitToDisplay(RenderTarget, TextureFilter),
  DrawMesh(GraphicsHandle, PrimitiveTopology, usize, usize),
}

impl CommandBuffer {
  pub fn new() -> Self {
    Self {
      commands: VecDeque::new(),
    }
  }

  pub fn clear_color_buffer(&mut self, color: Color) {
    self.commands.push_back(Command::ClearColor(color));
  }

  pub fn clear_depth_buffer(&mut self) {
    self.commands.push_back(Command::ClearDepth);
  }

  pub fn set_viewport_size(&mut self, viewport_size: (usize, usize)) {
    self.commands.push_back(Command::SetViewportSize(viewport_size));
  }

  pub fn set_blend_state(&mut self, blend_state: BlendState) {
    self.commands.push_back(Command::SetBlendState(blend_state));
  }

  pub fn set_scissor_mode(&mut self, scissor_mode: ScissorMode) {
    self.commands.push_back(Command::SetScissorMode(scissor_mode));
  }

  pub fn set_culling_mode(&mut self, culling_mode: CullingMode) {
    self.commands.push_back(Command::SetCullingMode(culling_mode));
  }

  pub fn set_render_target(&mut self, target: &RenderTarget) {
    self.commands.push_back(Command::SetTarget(target.clone()));
  }

  pub fn set_render_target_to_display(&mut self) {
    self.commands.push_back(Command::SetTargetToDisplay);
  }

  pub fn blit_to(&mut self, from: &RenderTarget, to: &RenderTarget, filter: TextureFilter) {
    self.commands.push_back(Command::Blit(from.clone(), to.clone(), filter));
  }

  pub fn blit_to_display(&mut self, target: &RenderTarget, filter: TextureFilter) {
    self.commands.push_back(Command::BlitToDisplay(target.clone(), filter));
  }

  pub fn draw_mesh<V>(&mut self, mesh: &Mesh<V>, topology: PrimitiveTopology, vertex_count: usize, index_count: usize) {
    let command = Command::DrawMesh(mesh.handle(), topology, vertex_count, index_count);
    self.commands.push_back(command);
  }

  pub fn flush(&mut self, graphics: &GraphicsServer) {
    while let Some(command) = self.commands.pop_front() {
      self.execute_command(command, graphics);
    }
  }

  fn execute_command(&mut self, command: Command, graphics: &GraphicsServer) {
    match command {
      Command::ClearColor(color) => graphics.clear_color_buffer(color),
      Command::ClearDepth => graphics.clear_depth_buffer(),
      Command::SetViewportSize(viewport_size) => graphics.set_viewport_size(viewport_size),
      Command::SetBlendState(blend_state) => graphics.set_blend_state(blend_state),
      Command::SetScissorMode(scissor_mode) => graphics.set_scissor_mode(scissor_mode),
      Command::SetCullingMode(culling_mode) => graphics.set_culling_mode(culling_mode),
      Command::SetTarget(target) => graphics.set_active_render_target(target.handle()),
      Command::SetTargetToDisplay => graphics.set_default_render_target(),
      Command::Blit(from, to, filter) => {
        let source_color = from.color_attachment();
        let dest_color = to.color_attachment();

        let source = Rectangle::from_corner_points(0, 0, source_color.width() as i32, source_color.height() as i32);
        let dest = Rectangle::from_corner_points(0, 0, dest_color.width() as i32, dest_color.height() as i32);

        graphics.blit_render_target(from.handle(), to.handle(), &source, &dest, filter);
      }
      Command::BlitToDisplay(from, filter) => {
        let source_color = from.color_attachment();

        let (width, height) = graphics.get_viewport_size();

        let source = Rectangle::from_corner_points(0, 0, source_color.width() as i32, source_color.height() as i32);
        let dest = Rectangle::from_corner_points(0, 0, width as i32, height as i32);

        graphics.blit_render_target_to_display(from.handle(), &source, &dest, filter);
      }
      Command::DrawMesh(mesh, topology, vertex_count, index_count) => {
        graphics.draw_mesh(mesh, topology, vertex_count, index_count);
      }
    }
  }
}

/// Allows an object to be rendered via a [`RenderManager`].
///
/// Requires that the manager is configured for the associated context.
pub trait Renderable<C: RenderContext> {
  /// Renders this object via the associated [`RenderContext`].
  fn render(&self, context: &mut C);
}

/// A context for rendering operations.
///
/// A context contains the state useful for a particular kind of rendering operation, and also
/// exposes some basic lifecycle methods.
pub trait RenderContext: Sized + 'static {
  fn on_initialize(&mut self) {}
  fn on_before_with(&mut self) {}
  fn on_after_with(&mut self) {}
}

/// Describes how to build a [`RenderContext`] .
///
/// A descriptor is a factory for a render context, and contain configuration and shared data
/// that is usable in the creation of the context itself.
pub trait RenderContextDescriptor {
  /// The type of context that will be created by this descriptor.
  type Context: RenderContext;

  /// Creates the associated context.
  fn create(&self, graphics: &GraphicsServer) -> Self::Context;
}

/// A manager for `RenderContext`s.
///
/// A [`RenderContext`] encodes all of the required details for textures,
/// materials, render targets, shaders, necessary in a single invocation of some
/// rendering state.
pub struct RenderContextManager {
  graphics: GraphicsServer,
  contexts: AnyMap,
}

impl RenderContextManager {
  /// Creates a new render manager.
  pub fn new(graphics: &GraphicsServer) -> Self {
    Self {
      graphics: graphics.clone(),
      contexts: AnyMap::new(),
    }
  }

  /// Configures the manager with the given context.
  pub fn configure<D: RenderContextDescriptor>(&mut self, descriptor: D) {
    self.contexts.insert(descriptor.create(&self.graphics));
  }

  /// Renders the given object via the associated context.
  pub fn render<R: Renderable<C>, C: RenderContext>(&mut self, renderable: &R) {
    self.with(|context| {
      renderable.render(context);
    });
  }

  /// Acquires a context for the given descriptor.
  ///
  /// If the context cannot be acquired, the body will not be run.
  pub fn with<C: RenderContext>(&mut self, body: impl FnOnce(&mut C)) {
    if let Some(context) = self.contexts.get_mut::<C>() {
      context.on_before_with();
      body(context);
      context.on_after_with();
    }
  }

  /// Releases the given context from the manager.
  pub fn release<C: RenderContext>(&mut self) {
    self.contexts.remove::<C>();
  }

  /// Clears all contexts from the manager, resetting it to a clean state.
  pub fn reset(&mut self) {
    self.contexts.clear();
  }
}

/// A manager for `RenderPass`es.
///
/// Render passes are executed based on their `RenderPass::order()`.
#[derive(Default)]
pub struct RenderPassManager {
  passes: Vec<Box<dyn RenderPass>>,
  commands: CommandBuffer,
}

/// Represents a single render pass in a renderer.
///
/// Render passes are executed in order to create a single frame.
pub trait RenderPass {
  /// The order in which this pass should evaluate.
  ///
  /// Higher values are evaluated last.
  fn order(&self) -> usize;

  fn begin_frame(&mut self, commands: &mut CommandBuffer);
  fn render_frame(&mut self, commands: &mut CommandBuffer);
  fn end_frame(&mut self, commands: &mut CommandBuffer);
}

impl RenderPassManager {
  /// Creates a new renderer.
  pub fn new() -> Self {
    Self {
      passes: Vec::new(),
      commands: CommandBuffer::new(),
    }
  }

  /// Adds a `RenderPass` to the renderer.
  pub fn add_pass(&mut self, pass: impl RenderPass + 'static) {
    self.passes.push(Box::new(pass));
    self.passes.sort_by_key(|pass| pass.order());
  }

  /// Renders a single frame to the given `GraphicsServer`.
  pub fn render(&mut self, graphics: &GraphicsServer) {
    let commands = &mut self.commands;

    for pass in &mut self.passes {
      pass.begin_frame(commands);
    }

    for pass in &mut self.passes {
      pass.render_frame(commands);
    }

    for pass in &mut self.passes {
      pass.end_frame(commands);
    }

    commands.flush(graphics);
  }
}
