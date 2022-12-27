use surreal_core::utilities::{Variant, VariantKind};

/// Allows a type to be reflected and provide
/// information about it's properties and methods.
pub trait Reflect {
  fn get_properties() -> Vec<PropertyInfo>;
  fn get_functions() -> Vec<FunctionInfo>;

  fn get_property(&self, _name: &str) -> Result<Variant, PropertyError>;
  fn set_property(&mut self, _name: &str, _value: Variant) -> Result<(), PropertyError>;

  /// Calls a method on the underlying type by name, passing the given arguments
  fn call_function(&mut self, _name: &str, _args: &[Variant]) -> Result<Variant, CallError> {
    Err(CallError::FunctionDoesntExist)
  }
}

/// Contains information about a single property.
#[derive(Clone, Debug)]
pub struct PropertyInfo {
  pub name: String,
  pub kind: VariantKind,
}

/// Contains information about a single function.
#[derive(Clone, Debug)]
pub struct FunctionInfo {
  pub name: String,
}

/// Possible errors for modifying a property via reflection.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PropertyError {
  PropertyDoesntExist,
  PropertySetFailed,
}

/// Possible errors for calling a function via reflection.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CallError {
  FunctionDoesntExist,
  FunctionFailed,
}
