//! Rendering abstractions and pipelines.
//!
//! This is a series of components designed to make it simpler to build more complex render
//! pipelines than using the 'material', 'mesh', 'render targets' etc do alone.

use std::{
  any::{Any, TypeId},
  collections::HashMap,
};

use crate::{
  engine::GameTime,
  maths::{vec2, Matrix4x4, Plane, Vector3},
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
/// exposes some basic lifecycle methods. It's lazily constructed upon first use and remains
/// alive until the [`RenderContextManager`] is dropped.
pub trait RenderContext: Any {
  fn as_any(&self) -> &dyn Any;
  fn as_any_mut(&mut self) -> &mut dyn Any;
  fn on_initialize(&mut self) {}
  fn on_begin_with(&mut self) {}
  fn on_end_with(&mut self) {}
  fn on_begin_frame(&mut self) {}
  fn on_end_frame(&mut self) {}
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
  contexts: HashMap<TypeId, Box<dyn RenderContext>>,
}

impl RenderContextManager {
  /// Creates a new render manager.
  pub fn new(graphics: &GraphicsServer) -> Self {
    Self {
      graphics: graphics.clone(),
      contexts: HashMap::new(),
    }
  }

  /// Configures the manager with the given context.
  pub fn configure<D: RenderContextDescriptor>(&mut self, descriptor: D) {
    let key = TypeId::of::<D::Context>();
    let value = Box::new(descriptor.create(&self.graphics));

    self.contexts.insert(key, value);
  }

  /// Begins a new frame.
  pub fn begin_frame(&mut self) {
    for context in self.contexts.values_mut() {
      context.on_begin_frame();
    }
  }

  /// Ends the current frame.
  pub fn end_frame(&mut self) {
    for context in self.contexts.values_mut() {
      context.on_end_frame();
    }
  }

  /// Acquires a context for the given descriptor.
  ///
  /// If the context cannot be acquired, the body will not be run.
  pub fn with<C: RenderContext>(&mut self, body: impl FnOnce(&mut C)) {
    if let Some(context) = self.contexts.get_mut(&TypeId::of::<C>()) {
      let context = context.as_any_mut().downcast_mut::<C>().unwrap();

      context.on_begin_with();
      body(context);
      context.on_end_with();
    }
  }

  /// Renders the given object via the associated context.
  pub fn render<R: Renderable<C>, C: RenderContext>(&mut self, renderable: &R) {
    self.with(|context| {
      renderable.render(context);
    });
  }

  /// Releases the given context from the manager.
  pub fn release<C: RenderContext>(&mut self) {
    self.contexts.remove(&TypeId::of::<C>());
  }

  /// Clears all contexts from the manager, resetting it to a clean state.
  pub fn reset(&mut self) {
    self.contexts.clear();
  }
}

// TODO: global material properties for this particular frame?

/// A context for a single frame, for use in [`RenderPass`] operations in a [`RenderPipeline`].
pub struct RenderFrame<'a> {
  pub graphics: &'a GraphicsServer,
  pub arena: &'a FixedMemoryArena,
  pub scene: &'a dyn RenderScene,
  pub camera: &'a dyn RenderCamera,
  pub uniforms: &'a mut MaterialUniformSet,
  pub context_manager: &'a mut RenderContextManager,
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
  pub id: u64,
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

  /// Retrieves the projection-view matrix for this camera.
  fn projection_view(&self) -> Matrix4x4;
}

/// Provides culling information to a renderer for use in trivial rejection.
pub trait RenderScene {
  /// Culls and computes visible objects from the perspective of the given frustum.
  fn cull_visible_objects(&self, frustum: &CameraFrustum, results: &mut Vec<CullingResult>);

  // rendering callbacks
  fn on_begin_frame(&self, _frame: &mut RenderFrame) {}
  fn on_end_frame(&self, _frame: &mut RenderFrame) {}

  /// Renders the given object against the given context manager.
  fn render(&self, id: u64, manager: &mut RenderContextManager);
}

/// Represents a single render pass in a renderer.
pub trait RenderPass {
  fn begin_frame(&mut self, _frame: &mut RenderFrame) {}
  fn render_frame(&mut self, _frame: &mut RenderFrame) {}
  fn end_frame(&mut self, _frame: &mut RenderFrame) {}
}

/// Allow closures to acts as render passes, where appropriate.
impl<F: FnMut(&mut RenderFrame)> RenderPass for F {
  fn render_frame(&mut self, frame: &mut RenderFrame) {
    self(frame)
  }
}

/// A pipeline for rendering, based on [`RenderPass`]es.
pub struct RenderPipeline {
  graphics: GraphicsServer,
  arena: FixedMemoryArena,
  render_passes: Vec<Box<dyn RenderPass>>,
  context_manager: RenderContextManager,
  frame_uniforms: MaterialUniformSet,
}

impl RenderPipeline {
  /// Creates a new render pipeline.
  pub fn new(graphics: &GraphicsServer) -> Self {
    Self {
      graphics: graphics.clone(),
      arena: FixedMemoryArena::default(),
      render_passes: Vec::new(),
      context_manager: RenderContextManager::new(&graphics),
      frame_uniforms: MaterialUniformSet::default(),
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
  pub fn render(&mut self, scene: &dyn RenderScene, camera: &dyn RenderCamera, time: &GameTime) {
    // compute frustum for this frame, and collect visible objects
    let frustum = camera.compute_frustum();
    let mut visible_objects = Vec::new(); // TODO: use the graphics arena here?

    scene.cull_visible_objects(&frustum, &mut visible_objects);
    visible_objects.sort_by_key(|it| it.material_key);

    // render this frame
    self.context_manager.begin_frame();
    {
      // build context for this frame; pass details down to the render passes
      let mut frame = RenderFrame {
        graphics: &self.graphics,
        arena: &self.arena,
        scene,
        camera,
        context_manager: &mut self.context_manager,
        visible_objects: &visible_objects,
        uniforms: &mut self.frame_uniforms,
      };

      // TODO: make this more customizable?
      let (width, height) = self.graphics.get_viewport_size();

      frame.uniforms.set_uniform("u_viewportSize", vec2(width as f32, height as f32));
      frame.uniforms.set_uniform("u_time", vec2(time.delta_time, time.total_time));
      frame.uniforms.set_uniform("u_projectionView", &camera.projection_view());

      scene.on_begin_frame(&mut frame);

      for pass in &mut self.render_passes {
        pass.begin_frame(&mut frame);
      }

      for pass in &mut self.render_passes {
        pass.render_frame(&mut frame);
      }

      for pass in &mut self.render_passes {
        pass.end_frame(&mut frame);
      }

      scene.on_end_frame(&mut frame);
    }
    self.context_manager.end_frame();

    self.arena.reset();
    self.frame_uniforms.clear();
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
    clear_color: Option<Color>,
  }

  struct TransparentPass;
  struct GrabPass;
  struct PostEffectPass;
  struct CompositePass;

  impl RenderPass for OpaquePass {
    fn render_frame(&mut self, frame: &mut RenderFrame) {
      if let Some(color) = self.clear_color {
        frame.graphics.clear_color_buffer(color);
      }

      for visible_object in frame.visible_objects {
        if visible_object.material_key.flags.contains(MaterialFlags::OPAQUE) {
          frame.scene.render(visible_object.id, frame.context_manager);
        }
      }
    }
  }

  impl RenderPass for TransparentPass {
    fn render_frame(&mut self, frame: &mut RenderFrame) {
      for visible_object in frame.visible_objects {
        if visible_object.material_key.flags.contains(MaterialFlags::TRANSPARENT) {
          frame.scene.render(visible_object.id, frame.context_manager);
        }
      }
    }
  }

  impl RenderPass for GrabPass {
    fn render_frame(&mut self, frame: &mut RenderFrame) {
      for visible_object in frame.visible_objects {
        if visible_object.material_key.flags.contains(MaterialFlags::GRAB_PASS) {
          frame.scene.render(visible_object.id, frame.context_manager);
        }
      }
    }
  }

  impl RenderPass for PostEffectPass {
    fn render_frame(&mut self, _frame: &mut RenderFrame) {}
  }

  impl RenderPass for CompositePass {
    fn begin_frame(&mut self, _frame: &mut RenderFrame) {}
    fn end_frame(&mut self, _frame: &mut RenderFrame) {}
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
    clear_color: configuration.clear_color,
  });

  pipeline.add_pass(TransparentPass);
  pipeline.add_pass(GrabPass);
  pipeline.add_pass(PostEffectPass);
  pipeline.add_pass(CompositePass);

  pipeline
}
