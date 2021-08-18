use super::memory::Memory;
use super::lex::Token;
use crate::Error;

#[derive(Debug)]
pub enum Instruction {
  Loop(Vec<Instruction>),
  Increment,
  Decrement,
  MoveRight,
  MoveLeft,
  Write,
  Read
}

impl Instruction {
  pub fn run<T>(&self, memory: &mut impl Memory<T>) -> Result<(), Error> {
    match self {
      Instruction::Increment => memory.increment(),
      Instruction::Decrement => memory.decrement(),
      Instruction::MoveRight => memory.move_right(),
      Instruction::MoveLeft => memory.move_left(),
      Instruction::Write => memory.write(),
      Instruction::Read => memory.read(),
      Instruction::Loop(instructions) => {
        while !memory.is_null() {
          for instruction in instructions {
            instruction.run(memory)?;
          }
        }
        Ok(())
      }
    }
  }
}

pub fn parse(tokens: &Vec<Token>) -> Result<Vec<Instruction>, Error> {
  parse_inner(tokens, false, &mut 0)
}

fn parse_inner(tokens: &Vec<Token>, is_loop: bool, i: &mut usize) -> Result<Vec<Instruction>, Error> {
  let mut instructions = vec![];
  while *i < tokens.len() {
    instructions.push(match tokens[*i] {
      Token::Increment => Instruction::Increment,
      Token::Decrement => Instruction::Decrement,
      Token::MoveRight => Instruction::MoveRight,
      Token::MoveLeft => Instruction::MoveLeft,
      Token::Write => Instruction::Write,
      Token::Read => Instruction::Read,
      Token::EnterLoop => {
        *i += 1;
        Instruction::Loop(parse_inner(tokens, true, i)?)
      }
      Token::ExitLoop => if is_loop {
        return Ok(instructions);
      } else {
        return Err(Error::UnmatchedCloseLoop(*i));
      }
    });
    *i += 1;
  }
  if is_loop {
    Err(Error::UnmatchedOpenLoop(*i))
  } else {
    Ok(instructions)
  }
}