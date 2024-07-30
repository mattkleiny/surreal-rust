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
    self.services.get(&TypeId::of::<T>()).and_then(|service| {
      // We only need the first service of the given type.
      service.iter().next().and_then(|service| service.downcast_ref::<T>())
    })
  }

  /// Mutably resolves a service of a given type.
  pub fn resolve_mut<T: 'static>(&mut self) -> Option<&mut T> {
    self.services.get_mut(&TypeId::of::<T>()).and_then(|service| {
      // We only need the first service of the given type.
      service
        .iter_mut()
        .next()
        .and_then(|service| service.downcast_mut::<T>())
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_service_resolution() {
    let mut services = ServiceProvider::new();

    services.register(42);

    assert_eq!(services.resolve::<i32>(), Some(&42));
  }
}
