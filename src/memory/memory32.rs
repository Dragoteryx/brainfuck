use crate::{Args, Error, Memory};

#[derive(Debug)]
pub struct Memory32<'a> {
  memory: Vec<u32>,
  pointer: usize,
  args: &'a Args
}

impl Memory32<'_> {
  pub fn new<'a>(args: &'a Args) -> Memory32<'a> {
    Memory32 {
      memory: vec![0; args.memory_size.into()],
      pointer: 0,
      args
    }
  }
}

impl Memory<u32> for Memory32<'_> {
  fn args(&self) -> &Args {
    self.args
  }
  fn size(&self) -> usize {
    self.args.memory_size.into()
  }
  fn null() -> u32 {
    0
  }

  // pointer

  fn pointer(&self) -> usize {
    self.pointer
  }
  fn pointer_mut(&mut self) -> &mut usize {
    &mut self.pointer
  }

  // memory

  fn value(&self, pointer: usize) -> u32 {
    self.memory[pointer as usize]
  }
  fn value_mut(&mut self, pointer: usize) -> &mut u32 {
    &mut self.memory[pointer as usize]
  }
  fn value_is_null(&self, value: u32) -> bool {
    value == 0
  }

  fn add_value(&self, pointer: usize, n: u32) -> Result<u32, Error> {
    if self.args.no_overflows {
      match self.value(pointer).checked_add(n) {
        None => Err(Error::PositiveOverflow(pointer)),
        Some(ok) => Ok(ok)
      }
    } else {
      Ok(self.value(pointer).wrapping_add(n))
    }
  }
  fn sub_value(&self, pointer: usize, n: u32) -> Result<u32, Error> {
    if self.args.no_overflows {
      match self.value(pointer).checked_sub(n) {
        None => Err(Error::NegativeOverflow(pointer)),
        Some(ok) => Ok(ok)
      }
    } else {
      Ok(self.value(pointer).wrapping_sub(n))
    }
  }

  // conversions

  fn value_to_u32(value: u32) -> u32 {
    value
  }
  fn value_to_string(value: u32) -> String {
    value.to_string()
  }
  fn value_to_char(value: u32) -> Result<char, Error> {
    match char::from_u32(value) {
      None => Err(Error::InvalidUnicode(value)),
      Some(char) => Ok(char)
    }
  }
  fn char_to_value(char: char) -> Result<u32, Error> {
    Ok(char as u32)
  }
}