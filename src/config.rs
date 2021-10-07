//! Configuration utilities

pub trait ConfigProperty<T> {
  fn get(&self) -> &T;
  fn set(&mut self, value: T);
}