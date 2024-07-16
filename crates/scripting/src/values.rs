use common::{Color, Color32, Quat, StringName, Variant, Vec2, Vec3, Vec4};

/// A value that can be passed to and from a scripting language.
///
/// This is a new-type pattern over [`Variant`] to allow simple interop with
/// many different scripting language vendor crates.
#[repr(transparent)]
#[derive(Clone, Debug, PartialEq)]
pub struct ScriptValue(Variant);

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

impl ScriptValue {
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

macro_rules! impl_script_value {
  ($type:ty, $($arg:ident),*) => {
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
          $(Variant::$arg(value) => value.clone() as $type,)*
          _ => panic!("Invalid type conversion"),
        }
      }
    }
  };
}

impl_script_value!(bool, Bool);
impl_script_value!(u8, U8, U16, U32, U64, I8, I16, I32, I64);
impl_script_value!(u16, U8, U16, U32, U64, I8, I16, I32, I64);
impl_script_value!(u32, U8, U16, U32, U64, I8, I16, I32, I64);
impl_script_value!(u64, U8, U16, U32, U64, I8, I16, I32, I64);
impl_script_value!(i8, U8, U16, U32, U64, I8, I16, I32, I64);
impl_script_value!(i16, U8, U16, U32, U64, I8, I16, I32, I64);
impl_script_value!(i32, U8, U16, U32, U64, I8, I16, I32, I64);
impl_script_value!(i64, U8, U16, U32, U64, I8, I16, I32, I64);
impl_script_value!(f32, U8, U16, U32, U64, I8, I16, I32, I64, F32, F64);
impl_script_value!(f64, U8, U16, U32, U64, I8, I16, I32, I64, F32, F64);
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

  #[test]
  fn test_script_value_coercion() {
    let value = 1.0f32.to_script_value();
    let result = f64::from_script_value(&value);

    assert_eq!(result, 1.0);
  }
}
