//! GPU mesh abstractions.

#[derive(Clone, Copy, Debug)]
pub enum VertexAttrType {
  F32,
  I16,
  I8,
  U16,
  U8,
}

#[derive(Clone, Copy, Debug)]
pub struct VertexAttrDescriptor {
  pub size: usize,
  pub class: VertexAttrClass,
  pub attr_type: VertexAttrType,
  pub stride: usize,
  pub offset: usize,
  pub divisor: u32,
  pub buffer_index: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VertexAttrClass {
  Float,
  FloatNorm,
  Int,
}