use surreal::utilities::{Variant, VariantKind};

/// Allows a type to be reflected over, and provides information about it's properties and methods.
pub trait Reflect {
  fn properties() -> &'static Vec<PropertyInfo>;
  fn methods() -> &'static Vec<MethodInfo>;

  /// Reads a property from the object.
  fn get_property(&self, name: impl AsRef<str>) -> Result<Variant, PropertyError>;

  /// Writes a property to the object.
  fn set_property(&mut self, name: impl AsRef<str>, value: Variant) -> Result<(), PropertyError>;

  /// Calls a method on the underlying type by name, passing the given arguments
  fn call_method(&mut self, _name: &str, _args: &[Variant]) -> Result<Variant, MethodCallError> {
    Err(MethodCallError::FunctionDoesntExist)
  }
}

/// Contains information about a single property.
#[derive(Clone, Debug)]
pub struct PropertyInfo {
  pub name: String,
  pub kind: VariantKind,
}

/// Contains information about a single method.
#[derive(Clone, Debug)]
pub struct MethodInfo {
  pub name: String,
}

/// Possible errors for modifying a property via [`Reflect`].
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PropertyError {
  PropertyDoesntExist,
  PropertySetFailed,
}

/// Possible errors for calling a method via [`Reflect`].
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MethodCallError {
  FunctionDoesntExist,
  FunctionFailed,
}
