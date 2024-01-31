/// Allows for the copying and pasting of text.
pub trait Clipboard {
  /// Returns the contents of the clipboard.
  fn get(&self) -> Option<String>;

  /// Sets the contents of the clipboard.
  fn set(&mut self, text: String);
}
