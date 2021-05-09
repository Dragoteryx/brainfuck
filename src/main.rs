use std::io::{stdin, stdout, Write, Read};
use std::path::Path;
use std::env::args;
use std::fs;

#[derive(Debug)]
#[derive(Clone)]
enum Token {
  Increment,
  Decrement,
  MoveRight,
  MoveLeft,
  Write,
  Read,
  EnterLoop,
  ExitLoop
}

fn lex(program: &str) -> Vec<Token> {
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
  return tokens;
}

#[derive(Debug)]
enum Instruction {
  Loop(Vec<Instruction>),
  Increment,
  Decrement,
  MoveRight,
  MoveLeft,
  Write,
  Read
}

fn parse(tokens: &Vec<Token>) -> Result<Vec<Instruction>, String> {
  let mut intructions = vec![];
  let mut loop_start = 0;
  let mut loop_stack = 0;
  for (i, token) in tokens.iter().enumerate() {
    if loop_stack == 0 {
      if let Some(instruction) = match token {
        Token::Increment => Some(Instruction::Increment),
        Token::Decrement => Some(Instruction::Decrement),
        Token::MoveRight => Some(Instruction::MoveRight),
        Token::MoveLeft => Some(Instruction::MoveLeft),
        Token::Write => Some(Instruction::Write),
        Token::Read => Some(Instruction::Read),
        Token::EnterLoop => {
          loop_start = i;
          loop_stack = 1;
          None
        }
        Token::ExitLoop => {
          return Err(format!("unmatched close loop token at position #{}", i+1));
        }
      } {
        intructions.push(instruction);
      }
    } else {
      if let Token::EnterLoop = token {
        loop_stack += 1;
      } else if let Token::ExitLoop = token {
        loop_stack -= 1;
        if loop_stack == 0 {
          intructions.push(Instruction::Loop(parse(&tokens[loop_start+1..i].to_vec())?));
        }
      }
    }
  }
  return Ok(intructions);
}

#[derive(Debug)]
struct Memory {
  current: usize,
  slots: Vec<u8>
}

impl Memory {
  fn new(size: usize) -> Memory {
    return Memory {
      slots: vec![0; size],
      current: 0
    };
  }
  fn get_value(&self) -> u8 {
    return self.slots[self.current];
  }
  fn set_value(&mut self, value: u8) -> Result<(), String> {
    self.slots[self.current] = value;
    return Ok(());
  }
  fn increment(&mut self) -> Result<(), String> {
    self.slots[self.current] = self.get_value().wrapping_add(1);
    return Ok(());
  }
  fn decrement(&mut self) -> Result<(), String> {
    self.slots[self.current] = self.get_value().wrapping_sub(1);
    return Ok(());
  }
  fn move_right(&mut self) -> Result<(), String> {
    return if self.current < self.slots.len()-1 {
      self.current += 1;
      Ok(())
    } else {
      Err(String::from("reached the rightmost cell"))
    }
  }
  fn move_left(&mut self) -> Result<(), String> {
    return if self.current > 0 {
      self.current -= 1;
      Ok(())
    } else {
      Err(String::from("reached the leftmost cell"))
    }
  }
  fn write(&self) -> Result<(), String> {
    print!("{}", self.get_value() as char);
    return if let Err(_) = stdout().flush() {
      Err(String::from("couldn't write output"))
    } else {
      Ok(())
    }
  }
  fn read(&mut self) -> Result<(), String> {
    let mut input: [u8; 1] = [0];
    return if let Ok(()) = stdin().read_exact(&mut input) {
      Err(String::from("couldn't read input"))
    } else {
      self.set_value(input[0])
    }
  }
}

fn run(instructions: &Vec<Instruction>, memory: &mut Memory) -> Result<(), String> {
  for instruction in instructions {
    match instruction {
      Instruction::Increment => memory.increment(),
      Instruction::Decrement => memory.decrement(),
      Instruction::MoveRight => memory.move_right(),
      Instruction::MoveLeft => memory.move_left(),
      Instruction::Write => memory.write(),
      Instruction::Read => memory.read(),
      Instruction::Loop(inner) => {
        while memory.get_value() != 0 {
          run(inner, memory)?;
        }
        Ok(())
      }
    }?;
  }
  return Ok(());
}

fn main() {
  let args: Vec<String> = args().collect();
  if args.len() < 2 {
    eprintln!("Usage: brainfuck <file.bf>");
  } else if !Path::new(&args[1]).exists() {
    eprintln!("Cannot find file: {}", &args[1]);
  } else if let Ok(content) = fs::read_to_string(&args[1]) {
    let tokens = lex(&content);
    match parse(&tokens) {
      Ok(instructions) => {
        let mut memory = Memory::new(30000);
        if let Err(err) = run(&instructions, &mut memory) {
          eprintln!("Runtime error: {}", err);
        }
      }
      Err(err) => {
        eprintln!("Compile time error: {}", err);
      }
    }
  } else {
    eprintln!("An unknown error happened while reading the file.");
  }
}