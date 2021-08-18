use std::io::{stdin, stdout, Write};
use colored::Colorize;
use super::Args;

#[derive(Debug)]
pub struct Memory<'a> {
  cells: Vec<u32>,
  pointer: usize,
  args: &'a Args
}

impl Memory<'_> {
  pub fn new<'a>(args: &'a Args) -> Memory<'a> {
    Memory {
      cells: vec![0; args.size.into()],
      pointer: 0,
      args
    }
  }

  // read/write memory

  fn max_value(&self) -> u32 {
    if self.args.larger_cells {
      u32::MAX
    } else {
      255
    }
  }

  pub fn get_value(&self) -> u32 {
    self.cells[self.pointer]
  }
  pub fn set_value(&mut self, value: u32) {
    self.cells[self.pointer] = value;
  }

  pub fn increment(&mut self) -> Result<(), String> {
    if self.get_value() < self.max_value() {
      Ok(self.set_value(self.get_value()+1))
    } else if self.args.no_overflows {
      Err(String::from(format!("Cell {} positively overflowed", self.pointer.to_string().green())))
    } else {
      Ok(self.set_value(0))
    }
  }
  pub fn decrement(&mut self) -> Result<(), String> {
    if self.get_value() > 0 {
      Ok(self.set_value(self.get_value()-1))
    } else if self.args.no_overflows {
      Err(String::from(format!("Cell {} negatively overflowed", self.pointer.to_string().green())))
    } else {
      Ok(self.set_value(self.max_value()))
    }
  }

  // move operations

  pub fn move_right(&mut self) -> Result<(), String> {
    if self.pointer < self.cells.len()-1 {
      self.pointer += 1;
      Ok(())
    } else if self.args.wrap_around {
      self.pointer = 0;
      Ok(())
    } else {
      Err(String::from("Reached the rightmost cell"))
    }
  }
  pub fn move_left(&mut self) -> Result<(), String> {
    if self.pointer > 0 {
      self.pointer -= 1;
      Ok(())
    } else if self.args.wrap_around {
      self.pointer = self.cells.len()-1;
      Ok(())
    } else {
      Err(String::from("Reached the leftmost cell"))
    }
  }

  // IO operations

  pub fn write(&self) -> Result<(), String> {
    let value = self.get_value();
    if self.args.debug {
      match char::from_u32(value) {
        Some(char) => {
          println!("[pointer: {}, value: {}, character: '{}']",
            self.pointer.to_string().green(),
            self.get_value().to_string().green(),
            char.to_string().yellow()
          );
          Ok(())
        }
        None => {
          println!("[pointer: {}, value: {}, invalid character]",
            self.pointer.to_string().green(),
            self.get_value().to_string().green()
          );
          Ok(())
        }
      }
    } else {
      match char::from_u32(value) {
        None => Err(String::from(format!("{} isn't a valid Unicode scalar value", value.to_string().green()))),
        Some(char) => {
          print!("{}", char);
          if stdout().flush().is_err() {
            Err(String::from("Failed to write output"))
          } else {
            Ok(())
          }
        }
        
      }
    } 
  }

  pub fn read(&mut self) -> Result<(), String> {
    let mut input = String::new();
    match stdin().read_line(&mut input) {
      Err(_) => Err(String::from("Failed to read input")),
      Ok(size) => {
        if size == 0 {
          Err(String::from("Expected input, got none"))
        } else {
          let char = input.chars().nth(0).unwrap();
          let value = char as u32;
          if value > self.max_value() {
            Err(String::from(format!("Cells aren't large enough to store the character '{}', enable the '--larger-cells' option", char.to_string().yellow())))
          } else {
            Ok(self.set_value(value))
          }
        }
      }
    }
  }
}