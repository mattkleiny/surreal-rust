use crate::maths::{DVec2, DVec3, Vec2, Vec3};

/// A simple implementation of the ear clipping algorithm.
pub trait EarClipper {}

macro_rules! impl_ear_clipper {
  ($type:ty) => {
    impl EarClipper for [$type] {}
  };
}

impl_ear_clipper!(Vec2);
impl_ear_clipper!(DVec2);
impl_ear_clipper!(Vec3);
impl_ear_clipper!(DVec3);
