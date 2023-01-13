use std::any::TypeId;

use crate::{collections::FastHashMap, utilities::Object};

/// A service that can be accessed via a [`ServiceProvider`].
pub trait Service: Object {}

/// Blanket implementation of [`Service`] for any [`Object`].
impl<A: Object> Service for A {}

/// Allows accessing services by type.
pub trait ServiceProvider {
  /// Returns a reference to the [`Service`] of the given type.
  fn get_service<T: Service>(&self) -> Option<&T>;

  /// Returns a mutable reference to the [`Service`] of the given type.
  fn get_service_mut<T: Service>(&mut self) -> Option<&mut T>;

  /// Returns a reference to the [`Service`] of the given type, or creates it
  /// anew.
  fn get_service_or_default<T: Service + Default>(&mut self) -> &mut T;
}

/// Maintains service instances and provides access to them.
#[derive(Default)]
pub struct ServiceContainer {
  services: FastHashMap<TypeId, Box<dyn Service>>,
}

impl ServiceProvider for ServiceContainer {
  fn get_service<T: Service>(&self) -> Option<&T> {
    self
      .services
      .get(&TypeId::of::<T>())
      .and_then(|service| service.as_any().downcast_ref())
  }

  fn get_service_mut<T: Service>(&mut self) -> Option<&mut T> {
    self
      .services
      .get_mut(&TypeId::of::<T>())
      .and_then(|service| service.as_any_mut().downcast_mut())
  }

  fn get_service_or_default<T: Service + Default>(&mut self) -> &mut T {
    self
      .services
      .entry(TypeId::of::<T>())
      .or_insert_with(|| Box::new(T::default()))
      .as_any_mut()
      .downcast_mut()
      .unwrap()
  }
}

/// Builder pattern for [`ServiceContainer`]s.
#[derive(Default)]
pub struct ServiceContainerBuilder {
  services: FastHashMap<TypeId, Box<dyn Service>>,
}

impl ServiceContainerBuilder {
  /// Adds a [`Service`] to the container via a [`Default`] instance.
  pub fn add_service_default<T: Service + Default>(self) -> Self {
    self.add_service(T::default())
  }

  /// Adds a [`Service`] to the container via the given factory method.
  pub fn add_service_factory<T: Service>(self, factory: impl Fn() -> T) -> Self {
    self.add_service(factory())
  }

  /// Adds a [`Service`] to the container.
  pub fn add_service<T: Service>(mut self, service: T) -> Self {
    self.services.insert(TypeId::of::<T>(), Box::new(service));
    self
  }

  /// Builds the resultant [`ServiceContainer`].
  pub fn build(self) -> ServiceContainer {
    ServiceContainer { services: self.services }
  }
}

#[cfg(test)]
mod tests {
  use macros::Object;

  use super::*;
  use crate as surreal;

  #[derive(Object, Default)]
  struct TestService1;

  #[derive(Object, Default)]
  struct TestService2;

  #[derive(Object, Default)]
  struct TestService3;

  #[test]
  fn service_container_should_register_and_yield_services() {
    let container = ServiceContainerBuilder::default()
      .add_service_default::<TestService1>()
      .add_service_default::<TestService2>()
      .build();

    assert!(container.get_service::<TestService1>().is_some());
    assert!(container.get_service::<TestService2>().is_some());
    assert!(container.get_service::<TestService3>().is_none());
  }
}
