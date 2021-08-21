use crate::{Args, Error, Memory};

#[derive(Debug)]
pub struct Memory8<'a> {
  memory: Vec<u8>,
  pointer: u32,
  args: &'a Args
}

impl Memory8<'_> {
  pub fn new<'a>(args: &'a Args) -> Memory8<'a> {
    Memory8 {
      memory: vec![0; u32::from(args.memory_size) as usize],
      pointer: 0,
      args
    }
  }
}

impl Memory<u8> for Memory8<'_> {
  fn args(&self) -> &Args {
    self.args
  }
  fn size(&self) -> u32 {
    self.args.memory_size.into()
  }
  fn null() -> u8 {
    0
  }

  // pointer

  fn pointer(&self) -> u32 {
    self.pointer
  }
  fn pointer_mut(&mut self) -> &mut u32 {
    &mut self.pointer
  }

  // memory

  fn value(&self, pointer: u32) -> u8 {
    self.memory[pointer as usize]
  }
  fn value_mut(&mut self, pointer: u32) -> &mut u8 {
    &mut self.memory[pointer as usize]
  }
  fn value_is_null(&self, value: u8) -> bool {
    value == 0
  }

  fn add_value(&self, pointer: u32, n: u32) -> Result<u8, Error> {
    if self.args.no_overflows {
      if n > 255 {
        Err(Error::PositiveOverflow(pointer))
      } else {
        match self.value(pointer).checked_add(n as u8) {
          None => Err(Error::PositiveOverflow(pointer)),
          Some(ok) => Ok(ok)
        }
      }
    } else {
      Ok(self.value(pointer).wrapping_add((n%256) as u8))
    }
  }
  fn sub_value(&self, pointer: u32, n: u32) -> Result<u8, Error> {
    if self.args.no_overflows {
      if n > 255 {
        Err(Error::NegativeOverflow(pointer))
      } else {
        match self.value(pointer).checked_sub(n as u8) {
          None => Err(Error::NegativeOverflow(pointer)),
          Some(ok) => Ok(ok)
        }
      }
    } else {
      Ok(self.value(pointer).wrapping_sub((n%256) as u8))
    }
  }

  // conversions

  fn value_to_u32(value: u8) -> u32 {
    value as u32
  }
  fn value_to_string(value: u8) -> String {
    value.to_string()
  }
  fn value_to_char(value: u8) -> Result<char, Error> {
    Ok(value as char)
  }
  fn char_to_value(char: char) -> Result<u8, Error> {
    let value = char as u32;
    if value > u16::MAX as u32 {
      Err(Error::Requires32Bits(char))
    } else if value > u8::MAX as u32 {
      Err(Error::Requires16Bits(char))
    } else {
      Ok(value as u8)
    }
  }
}