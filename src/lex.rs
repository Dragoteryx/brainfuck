#[derive(Debug)]
pub enum Token {
  Increment,
  Decrement,
  MoveRight,
  MoveLeft,
  Write,
  Read,
  EnterLoop,
  ExitLoop
}

pub fn lex(program: &str) -> Vec<Token> {
  let mut tokens = vec![];
  for char in program.chars() {
    if let Some(token) = match char {
      '+' => Some(Token::Increment),
      '-' => Some(Token::Decrement),
      '>' => Some(Token::MoveRight),
      '<' => Some(Token::MoveLeft),
      '.' => Some(Token::Write),
      ',' => Some(Token::Read),
      '[' => Some(Token::EnterLoop),
      ']' => Some(Token::ExitLoop),
      _ => None
    } {
      tokens.push(token);
    }
  }
  tokens
}