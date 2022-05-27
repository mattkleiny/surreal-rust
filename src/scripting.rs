//! Scripting support for the engine.

pub use lua::*;

use crate::assets::{Asset, AssetContext, AssetLoader};
use crate::io::AsVirtualPath;

mod lua;

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

/// An asset loader for scripts.
pub struct ScriptLoader {
  server: ScriptServer,
}

impl ScriptLoader {
  /// Creates a new script loader.
  pub fn new(server: &ScriptServer) -> Self {
    Self {
      server: server.clone()
    }
  }
}

impl Asset for Script {
  type Loader = ScriptLoader;
}

impl AssetLoader<Script> for ScriptLoader {
  fn load(&self, context: &AssetContext) -> crate::Result<Script> {
    Script::from_path(&self.server, context.path)
  }
}