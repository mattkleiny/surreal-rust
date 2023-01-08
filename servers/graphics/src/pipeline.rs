//! Rendering pipeline abstractions.

use surreal::engine::GameTime;
use surreal::graphics::{MaterialUniformSet, RenderContextDescriptor, Renderer};
use surreal::maths::{vec2, Mat4, Plane, Vec3};
use surreal::scene::SceneGraph;

use super::GraphicsServer;

#[cfg(feature = "pipeline-highdef")]
pub mod highdef;
#[cfg(feature = "pipeline-universal")]
pub mod universal;

/// A context for a single frame, for use in [`RenderPass`] operations.
pub struct RenderFrame<'a> {
  pub graphics: &'a GraphicsServer,
  pub scene: &'a dyn RenderScene,
  pub camera: &'a dyn RenderCamera,
  pub uniforms: &'a mut MaterialUniformSet,
  pub context_manager: &'a mut Renderer,
  pub visible_objects: &'a Vec<CullingResult>,
}

/// A key used to order rendering of objects by the material in use.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct MaterialSortKey {
  pub flags: MaterialFlags,
}

bitflags::bitflags! {
  /// Flags denoting what sort of material is visible from a [`CullingResult`].
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
  pub material_key: MaterialSortKey,
}

/// A frustum of 6 planes representing the camera's viewport; used to cull objects.
#[derive(Default, Clone)]
pub struct CameraFrustum {
  pub position: Vec3,
  pub planes: [Plane; 6],
}

/// Provides camera information for use in a dedicated render pipeline.
pub trait RenderCamera {
  /// Computes the frustum information for this camera, for use in later rendering steps.
  fn compute_frustum(&self) -> CameraFrustum;

  /// Retrieves the projection-view matrix for this camera.
  fn projection_view(&self) -> Mat4;
}

/// Provides culling information to a renderer for use in trivial rejection.
pub trait RenderScene {
  /// Culls and computes visible objects from the perspective of the given frustum.
  fn cull_visible_objects(&self, frustum: &CameraFrustum, results: &mut Vec<CullingResult>);

  // rendering callbacks
  fn on_begin_frame(&self, _frame: &mut RenderFrame) {}
  fn on_end_frame(&self, _frame: &mut RenderFrame) {}

  /// Renders the given object against the given context manager.
  fn render(&self, id: u64, manager: &mut Renderer);
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
  render_passes: Vec<Box<dyn RenderPass>>,
  context_manager: Renderer,
  frame_uniforms: MaterialUniformSet,
}

impl RenderPipeline {
  /// Creates a new render pipeline.
  pub fn new(graphics: &GraphicsServer) -> Self {
    Self {
      graphics: graphics.clone(),
      render_passes: Vec::new(),
      context_manager: Renderer::new(&graphics),
      frame_uniforms: MaterialUniformSet::default(),
    }
  }

  /// Configures the pipeline with the given render context.
  pub fn configure<D: RenderContextDescriptor>(&mut self, descriptor: D) {
    self.context_manager.add_descriptor(descriptor);
  }

  /// Adds a `RenderPass` to the render pipeline.
  ///
  /// Passes are evaluated in order of insertion.
  pub fn add_pass(&mut self, pass: impl RenderPass + 'static) {
    self.render_passes.push(Box::new(pass));
  }

  /// Renders a single frame of the given scene to the given graphics server from the perspective of the given camera.
  pub fn render(&mut self, scene: &dyn RenderScene, camera: &dyn RenderCamera, time: GameTime) {
    // compute frustum for this frame, and collect visible objects
    let frustum = camera.compute_frustum();
    let mut visible_objects = Vec::new();

    scene.cull_visible_objects(&frustum, &mut visible_objects);
    visible_objects.sort_by_key(|it| it.material_key);

    // render this frame
    self.context_manager.begin_frame();
    {
      // build context for this frame; pass details down to the render passes
      let mut frame = RenderFrame {
        graphics: &self.graphics,
        scene,
        camera,
        context_manager: &mut self.context_manager,
        visible_objects: &visible_objects,
        uniforms: &mut self.frame_uniforms,
      };

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
    self.frame_uniforms.clear();
  }
}
