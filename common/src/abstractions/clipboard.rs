/// Allows for the copying and pasting of text.
pub trait Clipboard {
  /// Returns the contents of the clipboard.
  fn get_clipboard(&self) -> Option<String>;

  /// Sets the contents of the clipboard.
  fn set_clipboard(&mut self, text: String);
}
