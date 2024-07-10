use super::*;

/// Represents a vector space.
pub trait Space {
  /// The cardinality of the vector space.
  const CARDINALITY: usize;

  type Vector: Vector<Scalar = Self::Scalar>;
  type Scalar: Scalar;
}

macro_rules! impl_space {
  ($name:ident, $cardinality:expr, $vector:ty, $scalar:ty, $comment:literal) => {
    #[doc = $comment]
    pub struct $name;

    impl Space for $name {
      const CARDINALITY: usize = $cardinality;

      type Vector = $vector;
      type Scalar = $scalar;
    }
  };
}

impl_space!(R2, 2, Vec2, f32, "The 2-dimensional vector space.");
impl_space!(R3, 3, Vec3, f32, "The 3-dimensional vector space.");
impl_space!(R4, 4, Vec4, f32, "The 4-dimensional vector space.");

/// Represents a vector in some vector space.
pub trait Vector:
  Copy
  + Clone
  + Default
  + Identity
  + Add<Output = Self>
  + AddAssign
  + Add<Self::Scalar, Output = Self>
  + AddAssign<Self::Scalar>
  + Sub<Output = Self>
  + SubAssign
  + Sub<Self::Scalar, Output = Self>
  + SubAssign<Self::Scalar>
  + Mul<Self::Scalar, Output = Self>
  + MulAssign<Self::Scalar>
  + Div<Self::Scalar, Output = Self>
  + DivAssign<Self::Scalar>
  + Sized
{
  /// The type of the space that this vector is in.
  type Space: Space;

  /// The type of the scalar that this vector is composed of.
  type Scalar: Scalar;
}

macro_rules! impl_vector {
  ($name:ident, $space:ident, $scalar:ident) => {
    impl Identity for $name {
      const ZERO: Self = Self::splat($scalar::ZERO);
      const ONE: Self = Self::splat($scalar::ONE);
      const MIN: Self = Self::splat($scalar::MIN);
      const MAX: Self = Self::splat($scalar::MAX);
    }

    impl Vector for $name {
      type Space = $space;
      type Scalar = $scalar;
    }
  };
}

impl_vector!(Vec2, R2, f32);
impl_vector!(DVec2, R2, f64);
impl_vector!(Vec3, R3, f32);
impl_vector!(DVec3, R3, f64);
impl_vector!(Vec4, R4, f32);
impl_vector!(DVec4, R4, f64);
