//! Rendering abstractions and pipelines.
//!
//! This is a series of components designed to make it simpler to build more complex render
//! pipelines than using the 'material', 'mesh', 'render targets' etc do alone.

use crate::{
  collections::AnyMap,
  maths::{Matrix4x4, Plane, Vector3},
  utilities::FixedMemoryArena,
};

use super::*;

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

  /// Borrows a context from the manager.
  pub fn get<C: RenderContext>(&self) -> Option<&C> {
    self.contexts.get()
  }

  /// Mutably borrows a context from the manager.
  pub fn get_mut<C: RenderContext>(&mut self) -> Option<&mut C> {
    self.contexts.get_mut()
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
  pub distance_metric: f32,
  pub material_key: MaterialKey,
}

/// A frustum of 6 planes representing the camera's viewport; used to cull objects.
#[derive(Default, Clone)]
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

  /// Enqueues rendering operations for the given target object.
  fn render_object(&self, id: usize, manager: &mut RenderContextManager);
}

/// Represents a single render pass in a renderer.
pub trait RenderPass {
  fn begin_frame(&mut self, _frame: &mut RenderFrame) {}
  fn render_frame(&mut self, frame: &mut RenderFrame);
  fn end_frame(&mut self, _frame: &mut RenderFrame) {}
}

/// A pipeline for rendering, based on [`RenderPass`]es.
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

/// Configuration for the forward rendering pipeline.
pub struct ForwardConfiguration {
  /// The color to use when clearing the scene at the start of the frame, or None to not clear.
  pub clear_color: Option<Color>,

  /// The resolution of the screen to render at; it not specified we'll render at the
  /// display/viewport resoluition, otherwise a custom render target will be created and
  /// either upscaled or dowscaled depending on the resolution.
  pub render_resolution: Option<(usize, usize)>,
}

/// Creates a forward rendering pipeline using default render passes.
///
/// The forward pipeline consists of the following core stages:
///
/// * An opaque pass for materials that contain only opaque geometry and don't require blending.
/// * A transparent pass for materials that do require blending, ordered by their distance metric.
/// * A grab pass for materials that require a capture of the current frame.
/// * A post effect pass that applies post-processing steps in seqeunce.
/// * A compositing pass that takes the resultant image and present it to the display (possibly with upscaling).
///
/// Configuration for the pipeline is via the [`ForwardConfiguration`] struct.
pub fn create_forward_pipeline(graphics: &GraphicsServer, configuration: &ForwardConfiguration) -> RenderPipeline {
  use crate::prototype::{GeometryBatchDescriptor, SpriteBatchDescriptor};

  struct OpaquePass {
    graphics: GraphicsServer,
    clear_color: Option<Color>,
  }

  struct TransparentPass;
  struct GrabPass;
  struct PostEffectPass;
  struct CompositePass;

  impl RenderPass for OpaquePass {
    fn render_frame(&mut self, frame: &mut RenderFrame) {
      if let Some(color) = self.clear_color {
        self.graphics.clear_color_buffer(color);
      }

      for visible_object in frame.visible_objects {
        if visible_object.material_key.flags.contains(MaterialFlags::OPAQUE) {
          frame.scene.render_object(visible_object.id, frame.manager);
        }
      }
    }
  }

  impl RenderPass for TransparentPass {
    fn render_frame(&mut self, frame: &mut RenderFrame) {
      for visible_object in frame.visible_objects {
        if visible_object.material_key.flags.contains(MaterialFlags::TRANSPARENT) {
          frame.scene.render_object(visible_object.id, frame.manager);
        }
      }
    }
  }

  impl RenderPass for GrabPass {
    fn render_frame(&mut self, frame: &mut RenderFrame) {
      for visible_object in frame.visible_objects {
        if visible_object.material_key.flags.contains(MaterialFlags::GRAB_PASS) {
          frame.scene.render_object(visible_object.id, frame.manager);
        }
      }
    }
  }

  impl RenderPass for PostEffectPass {
    fn render_frame(&mut self, _frame: &mut RenderFrame) {}
  }

  impl RenderPass for CompositePass {
    fn render_frame(&mut self, _frame: &mut RenderFrame) {}
  }

  let mut pipeline = RenderPipeline::new(graphics);

  // configure the render contexts
  let resolution = configuration.render_resolution.unwrap_or((256, 144));

  pipeline.configure(SpriteBatchDescriptor {
    projection_view: Matrix4x4::orthographic(resolution.0 as f32, resolution.1 as f32, 0., 100.),
    ..Default::default()
  });

  pipeline.configure(GeometryBatchDescriptor {
    projection_view: Matrix4x4::orthographic(resolution.0 as f32, resolution.1 as f32, 0., 100.),
    ..Default::default()
  });

  // configure rendering passes
  pipeline.add_pass(OpaquePass {
    graphics: graphics.clone(),
    clear_color: configuration.clear_color,
  });

  pipeline.add_pass(TransparentPass);
  pipeline.add_pass(GrabPass);
  pipeline.add_pass(PostEffectPass);
  pipeline.add_pass(CompositePass);

  pipeline
}
