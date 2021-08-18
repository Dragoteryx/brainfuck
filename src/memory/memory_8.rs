use std::io::{stdin, stdout, Write};
use colored::Colorize;
use super::Memory;
use crate::Error;
use crate::Args;

#[derive(Debug)]
pub struct Memory8<'a> {
  cells: Vec<u8>,
  pointer: usize,
  args: &'a Args
}

impl Memory8<'_> {
  pub fn new<'a>(args: &'a Args) -> Memory8<'a> {
    Memory8 {
      cells: vec![0; args.memory_size.into()],
      pointer: 0,
      args
    }
  }
}

impl Memory<u8> for Memory8<'_> {
  fn get_value(&self) -> u8 {
    self.cells[self.pointer]
  }
  fn is_null(&self) -> bool {
    self.get_value() == 0
  }
  
  fn set_value(&mut self, value: u8) {
    self.cells[self.pointer] = value;
  }
  fn increment(&mut self) -> Result<(), Error> {
    if self.get_value() < u8::MAX {
      Ok(self.set_value(self.get_value()+1))
    } else if self.args.no_overflows {
      Err(Error::PositiveOverflow(self.pointer))
    } else {
      Ok(self.set_value(0))
    }
  }
  fn decrement(&mut self) -> Result<(), Error> {
    if self.get_value() > 0 {
      Ok(self.set_value(self.get_value()-1))
    } else if self.args.no_overflows {
      Err(Error::NegativeOverflow(self.pointer))
    } else {
      Ok(self.set_value(u8::MAX))
    }
  }

  // move operations

  fn move_right(&mut self) -> Result<(), Error> {
    if self.pointer < self.cells.len()-1 {
      self.pointer += 1;
      Ok(())
    } else if self.args.wrap_around {
      self.pointer = 0;
      Ok(())
    } else {
      Err(Error::RightMostCell)
    }
  }
  fn move_left(&mut self) -> Result<(), Error> {
    if self.pointer > 0 {
      self.pointer -= 1;
      Ok(())
    } else if self.args.wrap_around {
      self.pointer = self.cells.len()-1;
      Ok(())
    } else {
      Err(Error::LeftMostCell)
    }
  }

  // IO operations

  fn write(&self) -> Result<(), Error> {
    let value = self.get_value();
    let char = value as char;
    if self.args.debug {
      println!("{} [pointer: {}, value: {}, character: '{}']",
        "debug:".green(),
        self.pointer.to_string().green(),
        self.get_value().to_string().green(),
        char.to_string().yellow()
      );
      Ok(())
    } else {
      print!("{}", char);
      if stdout().flush().is_err() {
        Err(Error::WriteOutputFail)
      } else {
        Ok(())
      }
    } 
  }

  fn read(&mut self) -> Result<(), Error> {
    let mut input = String::new();
    match stdin().read_line(&mut input) {
      Err(_) => Err(Error::ReadInputFail),
      Ok(size) => {
        if size == 0 {
          Err(Error::NoInput)
        } else {
          let char = input.chars().nth(0).unwrap();
          let value = char as u32;
          if value > u16::MAX.into() {
            Err(Error::Requires32Bits(char))
          } else if value > u8::MAX.into() {
            Err(Error::Requires16Bits(char))
          } else {
            Ok(self.set_value(char as u8))
          }
        }
      }
    }
  }
}