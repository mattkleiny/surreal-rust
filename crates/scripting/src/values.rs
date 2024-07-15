use common::{Color, Color32, Quat, StringName, Variant, Vec2, Vec3, Vec4};

/// A local type that can be converted to a [`Variant`].
pub trait ToScriptVariant {
  /// Converts the type into a [`Variant`].
  fn to_script_variant(&self) -> Variant;
}

/// A type that can be converted from a [`Variant`].
pub trait FromScriptVariant {
  /// Converts a [`Variant`] into the type.
  fn from_script_variant(value: &Variant) -> Self;
}

macro_rules! impl_script_value {
  ($type:ty, $kind:ident) => {
    impl ToScriptVariant for $type {
      #[inline]
      fn to_script_variant(&self) -> Variant {
        Variant::from(self.clone())
      }
    }

    impl FromScriptVariant for $type {
      #[inline]
      fn from_script_variant(value: &Variant) -> Self {
        match &value {
          Variant::$kind(value) => value.clone(),
          _ => panic!("Variant is not convertible"),
        }
      }
    }
  };
}

impl_script_value!(bool, Bool);
impl_script_value!(u8, U8);
impl_script_value!(u16, U16);
impl_script_value!(u32, U32);
impl_script_value!(u64, U64);
impl_script_value!(i8, I8);
impl_script_value!(i16, I16);
impl_script_value!(i32, I32);
impl_script_value!(i64, I64);
impl_script_value!(f32, F32);
impl_script_value!(f64, F64);
impl_script_value!(String, String);
impl_script_value!(StringName, StringName);
impl_script_value!(Vec2, Vec2);
impl_script_value!(Vec3, Vec3);
impl_script_value!(Vec4, Vec4);
impl_script_value!(Quat, Quat);
impl_script_value!(Color, Color);
impl_script_value!(Color32, Color32);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_script_value_conversion() {
    let value = true.to_script_variant();
    let result = bool::from_script_variant(&value);

    assert_eq!(result, true);
  }
}
