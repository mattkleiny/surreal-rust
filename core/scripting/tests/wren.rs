use common::Variant;

#[test]
pub fn it_should_compile_and_execute_wren_programs() {
  let program = "1 + 2 * 3";

  let expression = surreal_scripting::lang::wren::parse(program).unwrap();
  let mut opcodes = surreal_scripting::runtime::compiler::compile_expression(&expression).unwrap();

  opcodes.push(surreal_scripting::runtime::Opcode::Return);

  println!("{:?}", opcodes);

  let mut machine = surreal_scripting::runtime::machine::VirtualMachine::default();
  let result = machine.execute(&opcodes).unwrap();

  assert_eq!(result, Some(Variant::I64(7)));
}
