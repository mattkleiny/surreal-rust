//! Rendering abstractions and pipelines.
//!
//! This is a series of components designed to make it simpler to build more complex render
//! pipelines than using the 'material', 'mesh', 'render targets' etc do alone.

use super::*;
use crate::{
  collections::AnyMap,
  maths::{Plane, Vector3},
  utilities::FixedMemoryArena,
};
use std::collections::VecDeque;

pub use deferred::*;
pub use forward::*;

mod deferred;
mod forward;

/// A command buffer encodes a set of instructions to be replayed against the graphics server.
///
/// Command buffers decouple the order of instructions from the execution of those instructions
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

/// A context for a single frame, for use in [`RenderPass`] operations in a [`RenderPipeline`].
pub struct RenderFrame<'a> {
  pub arena: &'a FixedMemoryArena,
  pub scene: &'a dyn RenderScene,
  pub camera: &'a dyn RenderCamera,
  pub manager: &'a mut RenderContextManager,
  pub visible_objects: &'a Vec<CullingResult>,
}

/// A key used to order rendering of objects by the material in use.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct MaterialKey {
  pub flags: MaterialFlags,
}

bitflags::bitflags! {
  /// Flags denoting what sort of material is visible from a `CullingResult`.
  pub struct MaterialFlags: u8 {
    const OPAQUE = 1 << 0;
    const TRANSPARENT = 1 << 1;
    const GRAB_PASS = 1 << 2;
  }
}

/// Represents the result of a single culling pass.
///
/// A result contains information on an object that was perceived to be visible to the camera.
pub struct CullingResult {
  pub id: usize,
  pub distance_metrics: f32,
  pub material_key: MaterialKey,
}

/// A frustum of 6 planes representing the camera's viewport; used to cull objects.
#[derive(Clone)]
pub struct CameraFrustum {
  pub position: Vector3<f32>,
  pub planes: [Plane<f32>; 6],
}

/// Provides camera information for use in a dedicated render pipeline.
pub trait RenderCamera {
  /// Computes the frustum information for this camera, for use in later rendering steps.
  fn compute_frustum(&self) -> CameraFrustum;
}

/// Provides culling information to a renderer for use in trivial rejection.
pub trait RenderScene {
  /// Culls and computes visible objects from the perspective of the given frustum.
  ///
  /// The results are to be collected into the given `Vec`.
  fn cull_visible_objects(&self, frustum: &CameraFrustum, results: &mut Vec<CullingResult>);
}

/// Represents a single render pass in a renderer.
pub trait RenderPass {
  fn begin_frame(&mut self, _context: &mut RenderFrame) {}
  fn render_frame(&mut self, context: &mut RenderFrame);
  fn end_frame(&mut self, _context: &mut RenderFrame) {}
}

/// A pipeline for rendering, based on a [`RenderPass`]es.
pub struct RenderPipeline {
  arena: FixedMemoryArena,
  render_passes: Vec<Box<dyn RenderPass>>,
  context_manager: RenderContextManager,
}

impl RenderPipeline {
  /// Creates a new render pipeline.
  pub fn new(graphics: &GraphicsServer) -> Self {
    Self {
      arena: FixedMemoryArena::default(),
      render_passes: Vec::new(),
      context_manager: RenderContextManager::new(&graphics),
    }
  }

  /// Configures the pipeline with the given render context.
  pub fn configure<D: RenderContextDescriptor>(&mut self, descriptor: D) {
    self.context_manager.configure(descriptor);
  }

  /// Adds a `RenderPass` to the render pipeline.
  ///
  /// Passes are evaluated in order of insertion.
  pub fn add_pass(&mut self, pass: impl RenderPass + 'static) {
    self.render_passes.push(Box::new(pass));
  }

  /// Renders a single frame of the given scene to the given graphics server from the perspective of the given camera.
  pub fn render(&mut self, scene: &dyn RenderScene, camera: &dyn RenderCamera) {
    // compute frustum for this frame, and collect visible objects
    let frustum = camera.compute_frustum();
    let mut visible_objects = Vec::new(); // TODO: use the graphics arena here?

    scene.cull_visible_objects(&frustum, &mut visible_objects);
    visible_objects.sort_by_key(|it| it.material_key);

    // build context for this frame; pass details down to the render passes
    let mut frame = RenderFrame {
      arena: &self.arena,
      scene,
      camera,
      manager: &mut self.context_manager,
      visible_objects: &visible_objects,
    };

    for pass in &mut self.render_passes {
      pass.begin_frame(&mut frame);
    }

    for pass in &mut self.render_passes {
      pass.render_frame(&mut frame);
    }

    for pass in &mut self.render_passes {
      pass.end_frame(&mut frame);
    }

    self.arena.reset();
  }
}
