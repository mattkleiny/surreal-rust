use chumsky::prelude::*;

/// An instruction for a `brainfuck` program.
#[derive(Clone)]
pub enum Instruction {
  Left,
  Right,
  Incr,
  Decr,
  Read,
  Write,
  Loop(Vec<Self>),
}

/// A sample [`Parser`] for Chumsky that parses `brainfuck` instructions.
pub fn parser() -> impl Parser<char, Vec<Instruction>, Error = Simple<char>> {
  recursive(|it| {
    choice((
      just('<').to(Instruction::Left),
      just('>').to(Instruction::Right),
      just('+').to(Instruction::Incr),
      just('-').to(Instruction::Decr),
      just(',').to(Instruction::Read),
      just('.').to(Instruction::Write),
      it.delimited_by(just('['), just(']')).map(Instruction::Loop),
    ))
    .repeated()
  })
}
