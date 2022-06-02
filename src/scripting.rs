//! Scripting support for the engine.
//!
//! The eventual goal for scripting in Surreal is to provide a common runtime
//! for all manner of different scripting languages, each with a unified API.
//!
//! Allowing interoperation between different languages allows for the application
//! of the best tool for a particular job whilst still allowing the engine to
//! take care of optimization and management.

use crate::assets::{Asset, AssetContext, AssetLoader};
use crate::io::AsVirtualPath;

pub use compiler::*;
pub use languages::*;
pub use virtualmachine::*;

mod compiler;
mod languages;
mod virtualmachine;

/// An opaque handle to resource in the scripting subsystem.
pub type ScriptHandle = crate::collections::ArenaIndex;

/// The scripting server implementation.
pub type ScriptServer = std::rc::Rc<Box<dyn ScriptBackend>>;

/// Represents a graphical resource that possesses a `ScriptHandle`.
pub trait ScriptResource {
  fn handle(&self) -> ScriptHandle;
}

/// Represents a server implementation for the underlying scripting subsystem.
///
/// This is a high-level abstraction that makes use of 'opaque' handles to hide away implementation
/// details. The server is intended to be a low-level implementation abstraction.
pub trait ScriptBackend {
  fn create_script(&self) -> ScriptHandle;
  fn update_script(&self, script: ScriptHandle, code: String);
  fn execute_script(&self, script: ScriptHandle);
  fn delete_script(&self, script: ScriptHandle);
}

/// A managed wrapper for script execution.
#[derive(Clone)]
pub struct Script {
  server: ScriptServer,
  handle: ScriptHandle,
}

impl Script {
  /// Creates a new blank script.
  pub fn new(server: &ScriptServer) -> Self {
    Self {
      server: server.clone(),
      handle: server.create_script(),
    }
  }

  /// Loads a script from the given file path.
  pub fn from_path(server: &ScriptServer, path: impl AsVirtualPath) -> crate::Result<Self> {
    let mut script = Self::new(server);
    let code = path.as_virtual_path().read_all_text()?;

    script.set_code(code);

    Ok(script)
  }

  /// Sets script code from the given string.
  pub fn set_code(&mut self, code: String) {
    self.server.update_script(self.handle, code);
  }

  /// Executes the script.
  pub fn execute(&self) {
    self.server.execute_script(self.handle);
  }
}

impl Drop for Script {
  /// Deletes the script from the server.
  fn drop(&mut self) {
    self.server.delete_script(self.handle);
  }
}

/// An `AssetLoader` for `Script`s.
pub struct ScriptLoader {
  pub server: ScriptServer,
}

impl Asset for Script {
  type Loader = ScriptLoader;
}

impl AssetLoader<Script> for ScriptLoader {
  fn load(&self, context: &AssetContext) -> crate::Result<Script> {
    Script::from_path(&self.server, context.path)
  }
}
