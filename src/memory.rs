use std::io::{stdin, stdout, Write, Read};

#[derive(Clone, Debug)]
pub struct Memory {
  pointer: usize,
  cells: Vec<u8>,
  wrap: bool,
  debug: bool
}

impl Memory {
  pub fn new(size: usize, wrap: bool, debug: bool) -> Memory {
    Memory {
      cells: vec![0; size],
      pointer: 0,
      wrap,
      debug
    }
  }

  // read/write memory

  pub fn get_value(&self) -> u8 {
    self.cells[self.pointer]
  }
  pub fn set_value(&mut self, value: u8) {
    self.cells[self.pointer] = value;
  }
  pub fn increment(&mut self) {
    self.set_value(self.get_value().wrapping_add(1))
  }
  pub fn decrement(&mut self) {
    self.set_value(self.get_value().wrapping_sub(1))
  }

  // move operations

  pub fn move_right(&mut self) -> Result<(), String> {
    if self.pointer < self.cells.len()-1 {
      self.pointer += 1;
      Ok(())
    } else if self.wrap {
      self.pointer = 0;
      Ok(())
    } else {
      Err(String::from("reached the rightmost cell"))
    }
  }
  pub fn move_left(&mut self) -> Result<(), String> {
    if self.pointer > 0 {
      self.pointer -= 1;
      Ok(())
    } else if self.wrap {
      self.pointer = self.cells.len()-1;
      Ok(())
    } else {
      Err(String::from("reached the leftmost cell"))
    }
  }

  // IO operations

  pub fn write(&self) -> Result<(), String> {
    if self.debug {
      println!("[pointer: {}, value: {}, character: {}]", self.pointer, self.get_value(), self.get_value() as char);
      Ok(())
    } else {
      print!("{}", self.get_value() as char);
      if stdout().flush().is_err() {
        Err(String::from("couldn't write output"))
      } else {
        Ok(())
      }
    } 
  }
  pub fn read(&mut self) -> Result<(), String> {
    let mut input = [0];
    if stdin().read_exact(&mut input).is_err() {
      Err(String::from("couldn't read input"))
    } else {
      self.set_value(input[0]);
      Ok(())
    }
  }
}