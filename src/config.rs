pub trait Config<T> {
  fn get(&self) -> &T;
  fn set(&mut self, value: T);
}

struct ConfigVar<T> {
  value: T
}

impl<T> Config<T> for ConfigVar<T> {
  fn get(&self) -> &T {
    &self.value
  }

  fn set(&mut self, value: T) {
    self.value = value
  }
}