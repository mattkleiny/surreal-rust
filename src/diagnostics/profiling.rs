#[macro_export]
#[allow(unused_macros)]
macro_rules! profile {
  ($remain:tt*) => {};
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_profile_an_operation() {
    profile! {

    }
  }
}