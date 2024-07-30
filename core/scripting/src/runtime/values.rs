use common::{Color, Color32, Quat, StringName, Variant, Vec2, Vec3, Vec4};

/// Wraps any value that can be stored in a script.
///
/// This is a thin wrapper around the `Variant` type.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ScriptValue(pub Variant);

/// Represents a value that can be converted to a [`ScriptValue`].
pub trait ToScriptValue {
  fn to_script_value(self) -> ScriptValue;
}

/// Represents a value that can be converted from a [`ScriptValue`].
pub trait FromScriptValue {
  fn from_script_value(value: &ScriptValue) -> Self;
}

macro_rules! impl_script_value {
  ($type:ty, $variant:tt) => {
    impl ToScriptValue for &$type {
      #[inline]
      fn to_script_value(self) -> ScriptValue {
        ScriptValue(Variant::from(self.clone()))
      }
    }

    impl FromScriptValue for $type {
      #[inline]
      fn from_script_value(value: &ScriptValue) -> Self {
        match &value.0 {
          Variant::$variant(value) => value.clone(),
          _ => panic!("Invalid variant"),
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
  fn test_basic_type_conversion() {
    let base_value = 42u32;
    let script_value = base_value.to_script_value();

    assert_eq!(base_value, u32::from_script_value(&script_value));
  }
}
