use surreal_scripting::lang::{ScriptLanguage, BASIC};

#[test]
fn test_basic_program() {
  let code = r"
    1 REM This is a comment
    2 PRINT 'Hello, world!'
    3 PRINT 1 + 2
    4 PRINT 1 - 2
    5 PRINT 1 * 2
    6 PRINT 1 / 2

    10 LET A = 1
    11 LET B = 2

    20 PRINT A + B
  ";

  let _module = BASIC.parse_code(code).unwrap();
}
