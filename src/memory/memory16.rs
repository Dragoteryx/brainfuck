use crate::{Args, Error, Memory};

#[derive(Debug)]
pub struct Memory16<'a> {
  memory: Vec<u16>,
  pointer: u32,
  args: &'a Args
}

impl Memory16<'_> {
  pub fn new<'a>(args: &'a Args) -> Memory16<'a> {
    Memory16 {
      memory: vec![0; u32::from(args.memory_size) as usize],
      pointer: 0,
      args
    }
  }
}

impl Memory<u16> for Memory16<'_> {
  fn args(&self) -> &Args {
    self.args
  }
  fn size(&self) -> u32 {
    self.args.memory_size.into()
  }
  fn null() -> u16 {
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

  fn value(&self, pointer: u32) -> u16 {
    self.memory[pointer as usize]
  }
  fn value_mut(&mut self, pointer: u32) -> &mut u16 {
    &mut self.memory[pointer as usize]
  }
  fn value_is_null(&self, value: u16) -> bool {
    value == 0
  }

  fn add_value(&self, pointer: u32, n: u32) -> Result<u16, Error> {
    if self.args.no_overflows {
      if n > 0xFFFF {
        Err(Error::PositiveOverflow(pointer))
      } else {
        match self.value(pointer).checked_add(n as u16) {
          None => Err(Error::PositiveOverflow(pointer)),
          Some(ok) => Ok(ok)
        }
      }
    } else {
      Ok(self.value(pointer).wrapping_add((n%0x10000) as u16))
    }
  }
  fn sub_value(&self, pointer: u32, n: u32) -> Result<u16, Error> {
    if self.args.no_overflows {
      if n > 0xFFFF {
        Err(Error::NegativeOverflow(pointer))
      } else {
        match self.value(pointer).checked_sub(n as u16) {
          None => Err(Error::NegativeOverflow(pointer)),
          Some(ok) => Ok(ok)
        }
      }
    } else {
      Ok(self.value(pointer).wrapping_sub((n%0x10000) as u16))
    }
  }

  // conversions

  fn value_to_u32(value: u16) -> u32 {
    value as u32
  }
  fn value_to_string(value: u16) -> String {
    value.to_string()
  }
  fn value_to_char(value: u16) -> Result<char, Error> {
    match char::from_u32(value as u32) {
      None => Err(Error::InvalidUnicode(value as u32)),
      Some(char) => Ok(char)
    }
  }
  fn char_to_value(char: char) -> Result<u16, Error> {
    let value = char as u32;
    if value > u16::MAX as u32 {
      Err(Error::Requires32Bits(char))
    } else {
      Ok(value as u16)
    }
  }
}