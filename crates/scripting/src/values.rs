use common::{FromVariant, ToVariant, Variant};

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
  /// Creates a new `ScriptValue` from a `Variant` value.
  #[inline]
  pub const fn new(value: Variant) -> Self {
    Self(value)
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

/// Allow any variant type as a script value.
impl<V: FromVariant> FromScriptValue for V {
  #[inline]
  fn from_script_value(value: &ScriptValue) -> Self {
    V::from_variant(value.0.clone())
  }
}

/// Allow any script value as a variant type.
impl<V: ToVariant> ToScriptValue for V {
  #[inline]
  fn to_script_value(&self) -> ScriptValue {
    ScriptValue(self.to_variant())
  }
}

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
    let result = i32::from_script_value(&value);

    assert_eq!(result, 1);
  }
}
