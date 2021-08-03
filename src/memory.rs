use std::io::{stdin, stdout, Write, Read};

#[derive(Debug)]
pub struct Memory {
  pointer: usize,
  cells: Vec<u8>,
  wrap: bool
}

impl Memory {
  pub fn new(size: usize, wrap: bool) -> Memory {
    Memory {
      cells: vec![0; size],
      pointer: 0,
      wrap
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
    print!("{}", self.get_value() as char);
    if let Err(_) = stdout().flush() {
      Err(String::from("couldn't write output"))
    } else {
      Ok(())
    }
  }
  pub fn read(&mut self) -> Result<(), String> {
    let mut input = [0];
    if let Ok(()) = stdin().read_exact(&mut input) {
      Err(String::from("couldn't read input"))
    } else {
      self.set_value(input[0]);
      Ok(())
    }
  }
}