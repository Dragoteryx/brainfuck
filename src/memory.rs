use std::fmt::Debug;
use crate::Error;

mod memory8; pub use memory8::Memory8;
mod memory16; pub use memory16::Memory16;
mod memory32; pub use memory32::Memory32;

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