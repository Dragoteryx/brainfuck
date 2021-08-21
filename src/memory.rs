use std::io::{stdin, stdout, Write};
use crate::{Args, Error};
use colored::Colorize;
use std::fmt::Debug;

// memory

mod memory8; pub use memory8::*;
mod memory16; pub use memory16::*;
mod memory32; pub use memory32::*;

pub trait Memory<T>: Debug {
  fn args(&self) -> &Args;
  fn size(&self) -> u32;
  fn null() -> T;

  // pointer

  fn pointer(&self) -> u32;
  fn pointer_mut(&mut self) -> &mut u32;

  fn add_pointer(&self, n: u32) -> Result<u32, Error> {
    if self.size() > self.pointer() + n {
      Ok(self.pointer() + n)
    } else if self.args().wrap_around {
      Ok((self.pointer() + n) % self.size())
    } else {
      Err(Error::RightMostCell)
    }
  }
  fn sub_pointer(&self, n: u32) -> Result<u32, Error> {
    if self.pointer() >= n {
      Ok(self.pointer() - n)
    } else if self.args().wrap_around {
      Ok(self.size() - self.pointer() - n)
    } else {
      Err(Error::LeftMostCell)
    }
  }

  fn incr_pointer(&mut self, n: u32) -> Result<(), Error> {
    *self.pointer_mut() = self.add_pointer(n)?;
    Ok(())
  }
  fn decr_pointer(&mut self, n: u32) -> Result<(), Error> {
    *self.pointer_mut() = self.sub_pointer(n)?;
    Ok(())
  }

  // memory

  fn value(&self, pointer: u32) -> T;
  fn value_mut(&mut self, pointer: u32) -> &mut T;
  fn value_is_null(&self, value: T) -> bool;

  fn add_value(&self, pointer: u32, n: u32) -> Result<T, Error>;
  fn sub_value(&self, pointer: u32, n: u32) -> Result<T, Error>;

  fn incr_value(&mut self, pointer: u32, n: u32) -> Result<(), Error> {
    *self.value_mut(pointer) = self.add_value(pointer, n)?;
    Ok(())
  }
  fn decr_value(&mut self, pointer: u32, n: u32) -> Result<(), Error> {
    *self.value_mut(pointer) = self.sub_value(pointer, n)?;
    Ok(())
  }
  fn clear_value(&mut self, pointer: u32) -> Result<(), Error> {
    *self.value_mut(pointer) = Self::null();
    Ok(())
  }

  fn current(&self) -> T {
    self.value(self.pointer())
  }
  fn current_mut(&mut self) -> &mut T {
    self.value_mut(self.pointer())
  }
  fn current_is_null(&self) -> bool {
    self.value_is_null(self.current())
  }

  fn add_current(&self, n: u32) -> Result<T, Error> {
    self.add_value(self.pointer(), n)
  }
  fn sub_current(&self, n: u32) -> Result<T, Error> {
    self.sub_value(self.pointer(), n)
  }

  fn incr_current(&mut self, n: u32) -> Result<(), Error> {
    self.incr_value(self.pointer(), n)
  }
  fn decr_current(&mut self, n: u32) -> Result<(), Error> {
    self.decr_value(self.pointer(), n)
  }
  fn clear_current(&mut self) -> Result<(), Error> {
    self.clear_value(self.pointer())
  }

  // read / write

  fn value_to_u32(value: T) -> u32;
  fn value_to_string(value: T) -> String;
  fn value_to_char(value: T) -> Result<char, Error>;
  fn char_to_value(char: char) -> Result<T, Error>;

  fn write(&self) -> Result<(), Error> {
    if self.args().debug {
      match Self::value_to_char(self.current()) {
        Ok(char) => {
          println!("{} [pointer: {}, value: {}, character: '{}']",
            "debug:".green(),
            self.pointer().to_string().green(),
            Self::value_to_string(self.current()).green(),
            char.to_string().yellow()
          );
        }
        Err(_) => {
          println!("{} [pointer: {}, value: {}, invalid character]",
            "debug:".green(),
            self.pointer().to_string().green(),
            Self::value_to_string(self.current()).green(),
          );
        }
      }
      Ok(())
    } else {
      match Self::value_to_char(self.current()) {
        Err(err) => Err(err),
        Ok(char) => {
          print!("{}", char);
          Ok(())
        }
      }
    }
  }

  fn read(&mut self) -> Result<(), Error> {
    if stdout().flush().is_err() {
      return Err(Error::WriteOutputFail)
    }
    let mut input = String::new();
    match stdin().read_line(&mut input) {
      Err(_) => Err(Error::ReadInputFail),
      Ok(size) => {
        if size == 0 {
          Err(Error::NoInput)
        } else {
          let char = input.chars().nth(0).unwrap();
          *self.current_mut() = Self::char_to_value(char)?;
          Ok(())
        }
      }
    }
  }
}