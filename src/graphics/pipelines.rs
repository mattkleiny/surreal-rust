pub trait RenderCamera {}

pub trait RenderPipeline {
  type Renderer: Renderer;

  fn create_renderer(&self) -> Self::Renderer;
}

pub trait Renderer {
  fn begin_frame(&mut self);
  fn begin_camera(&mut self, camera: &impl RenderCamera);
  fn end_camera(&mut self, camera: &impl RenderCamera);
  fn end_frame(&mut self);
}

pub trait RenderPass {
  fn begin_frame(&mut self);
  fn begin_camera(&mut self, camera: &impl RenderCamera);
  fn end_camera(&mut self, camera: &impl RenderCamera);
  fn end_frame(&mut self);
}

pub struct ForwardRenderPipeline {}

impl RenderPipeline for ForwardRenderPipeline {
  type Renderer = ForwardRenderer;

  fn create_renderer(&self) -> Self::Renderer {
    ForwardRenderer {}
  }
}

pub struct ForwardRenderer {}

impl Renderer for ForwardRenderer {
  fn begin_frame(&mut self) {
    todo!()
  }

  fn begin_camera(&mut self, _camera: &impl RenderCamera) {
    todo!()
  }

  fn end_camera(&mut self, _camera: &impl RenderCamera) {
    todo!()
  }

  fn end_frame(&mut self) {
    todo!()
  }
}
