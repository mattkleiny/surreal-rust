use std::any::{Any, TypeId};

use crate::FastMultiMap;

/// Represents a provider of services.
///
/// This is a placeholder for a more complex service provider that can be used
/// to provide services to different parts of the engine.
#[derive(Default)]
pub struct ServiceProvider {
  services: FastMultiMap<TypeId, Box<dyn Any>>,
}

impl ServiceProvider {
  pub fn new() -> Self {
    Self {
      services: FastMultiMap::default(),
    }
  }

  /// Registers a service of a given type.
  pub fn register<T: 'static>(&mut self, service: T) {
    self.services.insert(TypeId::of::<T>(), Box::new(service));
  }

  /// Resolves a service of a given type.
  pub fn resolve<T: 'static>(&self) -> Option<&T> {
    todo!()
  }

  /// Mutably resolves a service of a given type.
  pub fn resolve_mut<T: 'static>(&self) -> Option<&mut T> {
    todo!()
  }
}
