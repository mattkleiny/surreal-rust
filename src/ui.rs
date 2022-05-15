//! A lightweight immediate mode UI framework.

/// Context for immediate mode UI rendering.
pub struct ImmediateModeContext {
  batch: ImmediateModeBatch,
}

impl ImmediateModeContext {
  /// Constructs a new blank immediate mode context.
  pub fn new() -> Self {
    Self {
      batch: ImmediateModeBatch::new(),
    }
  }

  /// Flushes changes to the context to the UI.
  pub fn flush(&mut self) {
    // TODO: implement me
  }
}

/// A batch of graphics operations to be handles in the immediate mode UI.
struct ImmediateModeBatch {}

impl ImmediateModeBatch {
  pub fn new() -> Self {
    Self {}
  }
}