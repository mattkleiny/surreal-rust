//! Describes high-level compilation mechanisms.

/// Represents a type capable of compiling instructions.
pub trait Compiler {
  type Instruction;

  /// Emits a single instruction for compilation into the given compiler.
  fn emit_instruction(&mut self, instruction: Self::Instruction);
}

/// Represents a type capable of emitting instructions for compilation.
pub trait Compilable {
  type Instruction;

  /// Emits all instructions for compilation for this object into the given compiler.
  fn emit_instructions(&self, compiler: &mut impl Compiler<Instruction=Self::Instruction>);
}