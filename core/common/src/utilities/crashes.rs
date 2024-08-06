//! Panic and crash handling.

/// Sets a handler for panics that occur in the application.
///
/// This handler will display a message to the user and invite them to report
/// the bug to the given URL.
pub fn configure_crash_handler(_version: &str, _bug_url: &str) {
  // TODO: implement me
  // std::panic::set_hook(Box::new({
  //   let version = version.to_string();
  //   let bug_url = bug_url.to_string();
  //
  //   move |c| panic_hook(c, &bug_url, &version)
  // }))
}
