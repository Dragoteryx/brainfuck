use std::fmt::Debug;
use crate::Error;

mod memory_8; pub use memory_8::Memory8;
mod memory_16; pub use memory_16::Memory16;
mod memory_32; pub use memory_32::Memory32;

pub trait Memory<T>: Debug {
  fn get_value(&self) -> T;
  fn is_null(&self) -> bool;

  fn set_value(&mut self, value: T);
  fn increment(&mut self) -> Result<(), Error>;
  fn decrement(&mut self) -> Result<(), Error>;

  fn move_right(&mut self) -> Result<(), Error>;
  fn move_left(&mut self) -> Result<(), Error>;

  fn write(&self) -> Result<(), Error>;
  fn read(&mut self) -> Result<(), Error>;
}