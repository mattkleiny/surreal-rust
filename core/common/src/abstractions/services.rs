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
  pub fn register<T: Any + 'static>(&mut self, service: T) {
    self.services.insert(TypeId::of::<T>(), Box::new(service));
  }

  /// Resolves a service of a given type.
  pub fn resolve<T: Any + 'static>(&self) -> Option<&T> {
    if let Some(services) = self.services.get(&TypeId::of::<T>()) {
      for service in services {
        if let Some(service) = service.downcast_ref::<T>() {
          return Some(service);
        }
      }
    }

    None
  }

  /// Resolves all services of a given type.
  pub fn resolve_all<T: Any + 'static>(&self) -> Vec<&T> {
    let mut result = Vec::new();

    if let Some(services) = self.services.get(&TypeId::of::<T>()) {
      for service in services {
        if let Some(service) = service.downcast_ref::<T>() {
          result.push(service);
        }
      }
    }

    result
  }

  /// Mutably resolves a service of a given type.
  pub fn resolve_mut<T: Any + 'static>(&mut self) -> Option<&mut T> {
    if let Some(services) = self.services.get_mut(&TypeId::of::<T>()) {
      for service in services {
        if let Some(service) = service.downcast_mut::<T>() {
          return Some(service);
        }
      }
    }

    None
  }

  /// Mutably resolves all services of a given type.
  pub fn resolve_mut_all<T: Any + 'static>(&mut self) -> Vec<&mut T> {
    let mut result = Vec::new();

    if let Some(services) = self.services.get_mut(&TypeId::of::<T>()) {
      for service in services {
        if let Some(service) = service.downcast_mut::<T>() {
          result.push(service);
        }
      }
    }

    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_service_single_usage() {
    let mut provider = ServiceProvider::new();

    provider.register(42);
    provider.register("Hello, World!");

    assert_eq!(provider.resolve::<i32>(), Some(&42));
  }

  #[test]
  fn test_service_multiple_usage() {
    let mut provider = ServiceProvider::new();

    provider.register(42);
    provider.register(43);
    provider.register(44);
    provider.register("Hello, World!");

    assert_eq!(provider.resolve::<i32>(), Some(&42));
    assert_eq!(provider.resolve_all::<i32>(), vec![&42, &43, &44]);
  }
}
