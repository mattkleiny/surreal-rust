//! BASIC language support for Surreal

use super::*;

/// The BASIC script language
pub struct BASIC;

impl ScriptLanguage for BASIC {
  fn load(_path: impl ToVirtualPath) -> Result<Script, ScriptError> {
    todo!()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load_basic_files_from_file_system() {
    let script = Script::from_path::<BASIC>("tests/test.basic").unwrap();

    assert_eq!(script.module.name, "test");
  }
}
