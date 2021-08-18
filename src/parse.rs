use super::memory::Memory;
use super::lex::Token;

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
  pub fn run(&self, memory: &mut Memory) -> Result<(), String> {
    match self {
      Instruction::Increment => memory.increment(),
      Instruction::Decrement => memory.decrement(),
      Instruction::MoveRight => memory.move_right(),
      Instruction::MoveLeft => memory.move_left(),
      Instruction::Write => memory.write(),
      Instruction::Read => memory.read(),
      Instruction::Loop(instructions) => {
        while memory.get_value() != 0 {
          for instruction in instructions {
            instruction.run(memory)?;
          }
        }
        Ok(())
      }
    }
  }
}

pub fn parse<'a, I: Iterator<Item = &'a Token>>(tokens: &mut I) -> Result<Vec<Instruction>, String> {
  parse_inner(tokens, false)
}

fn parse_inner<'a, I: Iterator<Item = &'a Token>>(tokens: &mut I, is_loop: bool) -> Result<Vec<Instruction>, String> {
  let mut instructions = vec![];
  while let Some(token) = tokens.next() {
    instructions.push(match token {
      Token::Increment => Instruction::Increment,
      Token::Decrement => Instruction::Decrement,
      Token::MoveRight => Instruction::MoveRight,
      Token::MoveLeft => Instruction::MoveLeft,
      Token::Write => Instruction::Write,
      Token::Read => Instruction::Read,
      Token::EnterLoop => Instruction::Loop(parse_inner(tokens, true)?),
      Token::ExitLoop => if is_loop {
        return Ok(instructions);
      } else {
        return Err(String::from("missing open loop token"));
      }
    });
  }
  if is_loop {
    Err(String::from("missing close loop token"))
  } else {
    Ok(instructions)
  }
}