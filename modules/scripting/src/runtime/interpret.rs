use super::ScriptRuntime;

/// Allows interpreting scripts.
///
/// This is a simple tree-walk interpreter.
#[derive(Default)]
pub struct Interpreter {}

impl ScriptRuntime for Interpreter {}
