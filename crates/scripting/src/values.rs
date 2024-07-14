use common::{Color, Color32, Quat, StringName, Variant, Vec2, Vec3, Vec4};

/// A value that can be passed to and from a scripting language.
///
/// This is a new-type pattern over [`Variant`] to allow simple interop with
/// many different scripting language vendor crates.
#[repr(transparent)]
#[derive(Clone, Debug, PartialEq)]
pub struct ScriptValue(pub Variant);

impl ScriptValue {
  /// Creates a new `ScriptValue` from a `Variant`.
  pub const fn new(variant: Variant) -> Self {
    Self(variant)
  }

  /// Returns a reference to the inner `Variant` value.
  #[inline]
  pub fn as_variant(&self) -> &Variant {
    &self.0
  }

  /// Returns the inner `Variant` value.
  #[inline]
  pub fn into_variant(self) -> Variant {
    self.0
  }
}

/// Allow conversion from [`Variant`].
impl From<Variant> for ScriptValue {
  #[inline]
  fn from(variant: Variant) -> Self {
    Self(variant)
  }
}

/// Allow conversion into [`Variant`].
impl From<ScriptValue> for Variant {
  #[inline]
  fn from(value: ScriptValue) -> Self {
    value.0
  }
}

/// A type that can be converted to a [`ScriptValue`].
pub trait ToScriptValue {
  /// Converts the type into a [`ScriptValue`].
  fn to_script_value(&self) -> ScriptValue;
}

/// A type that can be converted from a [`ScriptValue`].
pub trait FromScriptValue {
  /// Converts a [`ScriptValue`] into the type.
  fn from_script_value(value: &ScriptValue) -> Self;
}

macro_rules! impl_script_value {
  ($type:ty, $kind:ident) => {
    impl ToScriptValue for $type {
      #[inline]
      fn to_script_value(&self) -> ScriptValue {
        ScriptValue(Variant::from(self.clone()))
      }
    }

    impl FromScriptValue for $type {
      #[inline]
      fn from_script_value(value: &ScriptValue) -> Self {
        match &value.0 {
          Variant::$kind(value) => value.clone(),
          _ => panic!("ScriptValue is not convertible"),
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
    let value = true.to_script_value();
    let result = bool::from_script_value(&value);

    assert_eq!(result, true);
  }
}
