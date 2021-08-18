use std::io::{stdin, stdout, Write};
use colored::Colorize;
use super::Memory;
use crate::Error;
use crate::Args;

#[derive(Debug)]
pub struct Memory16<'a> {
  cells: Vec<u16>,
  pointer: usize,
  args: &'a Args
}

impl Memory16<'_> {
  pub fn new<'a>(args: &'a Args) -> Memory16<'a> {
    Memory16 {
      cells: vec![0; args.memory_size.into()],
      pointer: 0,
      args
    }
  }
}

impl Memory<u16> for Memory16<'_> {
  fn get_value(&self) -> u16 {
    self.cells[self.pointer]
  }
  fn is_null(&self) -> bool {
    self.get_value() == 0
  }
  
  fn set_value(&mut self, value: u16) {
    self.cells[self.pointer] = value;
  }
  fn increment(&mut self) -> Result<(), Error> {
    if self.get_value() < u16::MAX {
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
      Ok(self.set_value(u16::MAX))
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
    if self.args.debug {
      match char::from_u32(value as u32) {
        Some(char) => {
          println!("{} [pointer: {}, value: {}, character: '{}']",
            "debug:".green(),
            self.pointer.to_string().green(),
            self.get_value().to_string().green(),
            char.to_string().yellow()
          );
          Ok(())
        }
        None => {
          println!("{} [pointer: {}, value: {}, invalid character]",
            "debug:".green(),
            self.pointer.to_string().green(),
            self.get_value().to_string().green()
          );
          Ok(())
        }
      }
    } else {
      match char::from_u32(value as u32) {
        None => Err(Error::InvalidUnicode(value as u32)),
        Some(char) => {
          print!("{}", char);
          if stdout().flush().is_err() {
            Err(Error::WriteOutputFail)
          } else {
            Ok(())
          }
        }
        
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
          } else {
            Ok(self.set_value(char as u16))
          }
        }
      }
    }
  }
}