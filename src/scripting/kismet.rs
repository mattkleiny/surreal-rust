//! A visual scripting engine, inspired by Unreal's blueprints.

use crate::scripting::ScriptEngine;

pub struct KismetScriptEngine {}

impl KismetScriptEngine {
  pub fn new() -> Self {
    Self {}
  }
}

impl ScriptEngine for KismetScriptEngine {}
