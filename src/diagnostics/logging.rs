#[macro_export]
#[allow(unused_macros)]
macro_rules! log {
  ($system:ident, $message:ident) => {};
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_should_log_a_message() {
    unimplemented!()
  }
}