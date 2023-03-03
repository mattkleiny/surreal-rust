//! Binding utilties to allow managed execution of engine code from scripts.

/// Possible errors in the reflection system.
#[derive(thiserror:Error, Debug)]
pub enum ReflectError {
  #[error("type not registered")]
  TypeNotRegistered,
}

/// A database of types that can be used for a kind of 'runtime reflection'.
#[derive(Default)]
pub struct TypeDatabase {}

impl TypeDatabase {
  /// Registers a type with the database, allowing it to be used for binding.
  pub fn register_type<T: 'static>(&mut self, bindings: impl Fn(&mut impl TypeBindings<T>)) {
    let type_name = std::any::type_name::<T>();
    let type_id = std::any::TypeId::of::<T>();

    todo!();
  }
}

/// Allows type binding for some particular target type, [`T`].
pub trait TypeBindings<T> {
  /// Registers a method on the type.
  fn register_method<R = ()>(
    &mut self,
    name: &str,
    method: impl Fn(&mut T) -> Result<R, ReflectError>
  );

  /// Registers a property on the type.
  fn register_property<P>(
    &mut self,
    name: &str,
    getter: impl Fn(&T) -> Result<P, ReflectError>,
    setter: impl Fn(&mut T, P) -> Result<(), ReflectError>
  );
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_type_database() {
    let mut db = TypeDatabase::default();

    db.register_type::<i32>(|bindings| {
      bindings.register_method("add");
      bindings.register_property("value");
    });
  }
}
