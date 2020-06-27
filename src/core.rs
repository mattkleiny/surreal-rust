pub struct Database;

impl Database {
  pub fn register(&mut self, class: &impl Class) {
    unimplemented!()
  }
}

pub trait Class {}

pub struct Registration;

impl Registration {
  pub fn register_method(&mut self) {}
  pub fn register_property(&mut self) {}
}