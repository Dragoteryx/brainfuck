use std::io::{stdin, stdout, Write, Read};
use colored::Colorize;

use super::Args;

#[derive(Debug)]
pub struct Memory<'a> {
  cells: Vec<u8>,
  pointer: usize,
  args: &'a Args
}

impl Memory<'_> {
  pub fn new<'a>(args: &'a Args) -> Memory<'a> {
    Memory {
      cells: vec![0; args.size],
      pointer: 0,
      args
    }
  }

  // read/write memory

  pub fn get_value(&self) -> u8 {
    self.cells[self.pointer]
  }
  pub fn set_value(&mut self, value: u8) {
    self.cells[self.pointer] = value;
  }
  pub fn increment(&mut self) -> Result<(), String> {
    if self.get_value() < 255 {
      Ok(self.set_value(self.get_value()+1))
    } else if self.args.no_overflows {
      Err(String::from(format!("Cell {} positively overflowed", self.pointer)))
    } else {
      Ok(self.set_value(0))
    }
  }
  pub fn decrement(&mut self) -> Result<(), String> {
    if self.get_value() > 0 {
      Ok(self.set_value(self.get_value()-1))
    } else if self.args.no_overflows {
      Err(String::from(format!("Cell {} negatively overflowed", self.pointer)))
    } else {
      Ok(self.set_value(255))
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
    if self.args.debug {
      println!("[pointer: {}, value: {}, character: '{}']",
        self.pointer.to_string().green(),
        self.get_value().to_string().green(),
        (self.get_value() as char).to_string().yellow()
      );
      Ok(())
    } else {
      print!("{}", self.get_value() as char);
      if stdout().flush().is_err() {
        Err(String::from("Couldn't write output"))
      } else {
        Ok(())
      }
    } 
  }
  pub fn read(&mut self) -> Result<(), String> {
    let mut input = [0];
    if stdin().read_exact(&mut input).is_err() {
      Err(String::from("Couldn't read input"))
    } else {
      self.set_value(input[0]);
      Ok(())
    }
  }
}