use std::any::{Any, TypeId};

use crate::{reinterpret_cast, reinterpret_cast_mut, FastHashMap};

/// A provider for services.
pub trait ServiceProvider {
  /// Returns a reference to the service of the given type.
  fn get<T: 'static>(&self) -> Option<&T>;

  /// Returns a mutable reference to the service of the given type.
  fn get_mut<T: 'static>(&mut self) -> Option<&mut T>;
}

/// A collection of services.
#[derive(Default)]
pub struct ServiceCollection {
  services: FastHashMap<TypeId, Box<dyn Any>>,
}

impl ServiceCollection {
  /// Adds a new service to the collection.
  pub fn add_service<T: 'static>(&mut self, service: T) {
    self.services.insert(TypeId::of::<T>(), Box::new(service));
  }

  /// Clears all services from the collection.
  pub fn clear(&mut self) {
    self.services.clear();
  }
}

impl ServiceProvider for ServiceCollection {
  fn get<T: 'static>(&self) -> Option<&T> {
    self
      .services
      .get(&TypeId::of::<T>())
      .map(|service| unsafe { reinterpret_cast(service) as &Box<T> }.as_ref())
  }

  fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
    self
      .services
      .get_mut(&TypeId::of::<T>())
      .map(|service| unsafe { reinterpret_cast_mut(service) as &mut Box<T> }.as_mut())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Default)]
  struct TestService;

  #[test]
  fn test_basic_service_add_and_retrieve() {
    let mut services = ServiceCollection::default();

    services.add_service(TestService);

    services.get::<TestService>().unwrap();
    services.get_mut::<TestService>().unwrap();
  }
}
