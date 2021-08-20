use crate::{Memory, Token, Error};

#[derive(Debug, Clone, Copy)]
pub enum Edit {
  Increment(u32),
  Decrement(u32)
}

#[derive(Debug, Clone, Copy)]
pub enum Move {
  Right(usize),
  Left(usize)
}

#[derive(Debug)]
pub enum Instruction {
  Loop(Vec<Instruction>),
  EditOtherLoop(Move, Edit),
  MoveEdit(Move, Edit),
  EditOther(Move, Edit),
  Edit(Edit),
  Move(Move),
  Clear,
  Write,
  Read,
}

impl Instruction {
  pub fn run<T, M: Memory<T>>(&self, memory: &mut M) -> Result<(), Error> {
    //memory.write()?;
    //println!("{:?}", self);
    match self {
      &Instruction::Edit(Edit::Increment(n)) => memory.incr_current(n),
      &Instruction::Edit(Edit::Decrement(n)) => memory.decr_current(n),
      &Instruction::Move(Move::Right(n)) => memory.incr_pointer(n),
      &Instruction::Move(Move::Left(n)) => memory.decr_pointer(n),
      Instruction::Clear => memory.clear_current(),
      Instruction::Write => memory.write(),
      Instruction::Read => memory.read(),
      Instruction::Loop(instructions) => {
        while !memory.current_is_null() {
          for instruction in instructions {
            instruction.run(memory)?;
          }
        }
        Ok(())
      }
      Instruction::EditOtherLoop(mov, edit) => {
        let pointer = match mov {
          &Move::Right(n) => memory.add_pointer(n)?,
          &Move::Left(n) => memory.sub_pointer(n)?
        };
        match edit {
          &Edit::Increment(n) => memory.incr_value(pointer, n * M::value_to_u32(memory.current()))?,
          &Edit::Decrement(n) => memory.decr_value(pointer, n * M::value_to_u32(memory.current()))?
        }
        memory.clear_current()?;
        Ok(())
      }
      Instruction::MoveEdit(mov, edit) => {
        match mov {
          &Move::Right(n) => memory.incr_pointer(n)?,
          &Move::Left(n) => memory.decr_pointer(n)?
        }
        match edit {
          &Edit::Increment(n) => memory.incr_current(n)?,
          &Edit::Decrement(n) => memory.decr_current(n)?
        }
        Ok(())
      }
      Instruction::EditOther(mov, edit) => {
        let pointer = match mov {
          &Move::Right(n) => memory.add_pointer(n)?,
          &Move::Left(n) => memory.sub_pointer(n)?
        };
        match edit {
          &Edit::Increment(n) => memory.incr_value(pointer, n)?,
          &Edit::Decrement(n) => memory.decr_value(pointer, n)?
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
      Token::Increment => Instruction::Edit(Edit::Increment(1)),
      Token::Decrement => Instruction::Edit(Edit::Decrement(1)),
      Token::MoveRight => Instruction::Move(Move::Right(1)),
      Token::MoveLeft => Instruction::Move(Move::Left(1)),
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