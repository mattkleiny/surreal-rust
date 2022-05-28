use std::cell::RefCell;

use crate::collections::Arena;

use super::*;

/// A script backend implementation for Lua.
pub struct LuaScriptBackend {
  internal_state: RefCell<InternalState>,
}

/// Internal state for the Lua backend.
struct InternalState {
  scripts: Arena<LuaScriptState>,
}

/// Internal state for a single script.
struct LuaScriptState {
  lua: mlua::Lua,
  code: Option<String>,
}

impl LuaScriptBackend {
  /// Creates a new lua backend.
  pub fn new() -> ScriptServer {
    ScriptServer::new(Box::new(Self {
      internal_state: RefCell::new(InternalState {
        scripts: Arena::new(),
      }),
    }))
  }
}

impl ScriptBackend for LuaScriptBackend {
  fn create_script(&self) -> ScriptHandle {
    let mut state = self.internal_state.borrow_mut();
    
    state.scripts.add(LuaScriptState {
      lua: mlua::Lua::new(),
      code: None
    })
  }

  fn update_script(&self, script: ScriptHandle, code: String) {
    let mut state = self.internal_state.borrow_mut();

    if let Some(script) = state.scripts.get_mut(script) {
      script.code = Some(code);
    }
  }

  fn execute_script(&self, script: ScriptHandle) {
    let mut state = self.internal_state.borrow_mut();

    if let Some(script) = state.scripts.get_mut(script) {
      if let Some(code) = &script.code {
        script.lua.load(code).exec().expect("Failed to execute script")
      }
    }
  }

  fn delete_script(&self, script: ScriptHandle) {
    let mut state = self.internal_state.borrow_mut();

    state.scripts.remove(script);
  }
}