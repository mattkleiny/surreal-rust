pub struct Renderer {}

impl Renderer {
  pub fn new() -> Self {
    Self {}
  }

  pub fn add_pass(&mut self, stage: RenderStage, pass: Box<dyn RenderPass>) {
    unimplemented!()
  }

  pub fn render(&mut self, stage: RenderStage) {
    unimplemented!()
  }
}

pub trait RenderPass {
  fn render(&mut self);
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum RenderStage {
  BeforeAll,
  Opaque,
  Transparent,
  PostProcess,
  AfterAll,
}