/// A singleton that can be referenced statically in the application.
pub trait Singleton: Default + Send + Sync + 'static {
  /// Retrieves the static instance of this type.
  fn instance() -> &'static std::sync::Mutex<Self>;

  /// Locks and accesses an instance of the singleton.
  fn lock() -> std::sync::MutexGuard<'static, Self> {
    Self::instance().lock().unwrap()
  }
}

/// Declares a singleton instance of the given type.
///
/// The singleton can have no constructed dependencies, and is expected
/// to run in complete isolation of the rest of the application.
#[macro_export]
macro_rules! singleton {
  ($name:ident) => {
    impl crate::utilities::Singleton for $name {
      fn instance() -> &'static std::sync::Mutex<$name> {
        lazy_static::lazy_static! {
          static ref INSTANCE: std::sync::Mutex<$name> = std::sync::Mutex::new($name::default());
        }

        &INSTANCE
      }
    }
  };
}

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Default)]
  struct TestSingleton;

  impl TestSingleton {
    pub fn example_method(&self) -> u32 {
      42u32
    }
  }

  singleton!(TestSingleton);

  #[test]
  fn singleton_should_access_for_read_write_usage() {
    let test = TestSingleton::instance().lock().unwrap();

    assert_eq!(42, test.example_method());
  }
}
