use colored::Colorize;

#[derive(Debug)]
pub enum Error {
  // compilation
  UnmatchedCloseLoop(usize),
  UnmatchedOpenLoop(usize),

  // runtime
  PositiveOverflow(usize),
  NegativeOverflow(usize),
  RightMostCell,
  LeftMostCell,
  InvalidUnicode(u32),
  WriteOutputFail,
  NoInput,
  ReadInputFail,
  Requires32Bits(char),
  Requires16Bits(char)
}

impl Error {
  pub fn message(&self) -> String {
    match self {
      Error::UnmatchedCloseLoop(position) => String::from(format!("Unmatched close loop token at position {}", position.to_string().green())),
      Error::UnmatchedOpenLoop(position) => String::from(format!("Unmatched open loop token at position {}", position.to_string().green())),
      Error::PositiveOverflow(pointer) => String::from(format!("Cell {} positively overflowed", pointer.to_string().green())),
      Error::NegativeOverflow(pointer) => String::from(format!("Cell {} negatively overflowed", pointer.to_string().green())),
      Error::RightMostCell => String::from("Reached the rightmost cell"),
      Error::LeftMostCell => String::from("Reached the leftmost cell"),
      Error::InvalidUnicode(value) => String::from(format!("{} isn't a valid Unicode scalar value", value.to_string().green())),
      Error::WriteOutputFail => String::from("Failed to write output"),
      Error::NoInput => String::from("Expected input, got none"),
      Error::ReadInputFail => String::from("Failed to read input"),
      Error::Requires32Bits(char) => String::from(format!("Storing the character '{}' requires 32 bits", char.to_string().yellow())),
      Error::Requires16Bits(char) => String::from(format!("Storing the character '{}' requires 16 bits", char.to_string().yellow())),
    }
  }
}