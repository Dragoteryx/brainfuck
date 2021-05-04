use std::io::{stdin, stdout, Write, Read};
use std::path::Path;
use std::env::args;
use std::fs;

#[derive(Debug)]
#[derive(Clone)]
enum Token {
  MoveRight,
  MoveLeft,
  Increment,
  Decrement,
  Write,
  Read,
  EnterLoop,
  ExitLoop
}

fn lex(program: &str) -> Vec<Token> {
  let mut tokens = vec![];
  for char in program.chars() {
    if let Some(token) = match char {
      '>' => Some(Token::MoveRight),
      '<' => Some(Token::MoveLeft),
      '+' => Some(Token::Increment),
      '-' => Some(Token::Decrement),
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
  MoveRight,
  MoveLeft,
  Increment,
  Decrement,
  Write,
  Read
}

fn parse(tokens: &Vec<Token>) -> Vec<Instruction> {
  let mut intructions = vec![];
  let mut loop_start = 0;
  let mut loop_stack = 0;
  for (i, token) in tokens.iter().enumerate() {
    if loop_stack == 0 {
      if let Some(instruction) = match token {
        Token::MoveRight => Some(Instruction::MoveRight),
        Token::MoveLeft => Some(Instruction::MoveLeft),
        Token::Increment => Some(Instruction::Increment),
        Token::Decrement => Some(Instruction::Decrement),
        Token::Write => Some(Instruction::Write),
        Token::Read => Some(Instruction::Read),
        Token::ExitLoop => panic!("unmatched close loop token at position #{}", i+1),
        Token::EnterLoop => {
          loop_start = i;
          loop_stack = 1;
          None
        }
      } {
        intructions.push(instruction);
      }
    } else if let Token::EnterLoop = token {
      loop_stack += 1;
    } else if let Token::ExitLoop = token {
      loop_stack -= 1;
      if loop_stack == 0 {
        intructions.push(Instruction::Loop(parse(&tokens[loop_start+1..i].to_vec())));
      }
    }
  }
  return intructions;
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
    let value = self.get_value();
    return self.set_value(if value != 255 {
      value+1
    } else {
      0
    });
  }
  fn decrement(&mut self) -> Result<(), String> {
    let value = self.get_value();
    return self.set_value(if value != 0 {
      value-1
    } else {
      255
    });
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
    if let Err(err) = match instruction {
      Instruction::MoveRight => memory.move_right(),
      Instruction::MoveLeft => memory.move_left(),
      Instruction::Increment => memory.increment(),
      Instruction::Decrement => memory.decrement(),
      Instruction::Write => memory.write(),
      Instruction::Read => memory.read(),
      Instruction::Loop(inner) => {
        while memory.get_value() != 0 {
          if let Err(err) = run(inner, memory) {
            return Err(err);
          }
        }
        Ok(())
      }
    } {
      return Err(err)
    }
  }
  return Ok(());
}

fn main() {
  let args: Vec<String> = args().collect();
  if args.len() < 2 {
    println!("Usage: brainfuck <file.bf>");
  } else if !Path::new(&args[1]).exists() {
    println!("Cannot find file: {}", &args[1]);
  } else if let Ok(content) = fs::read_to_string(&args[1]) {
    let tokens = lex(&content);
    let instructions = parse(&tokens);
    let mut memory = Memory::new(30000);
    if let Err(err) = run(&instructions, &mut memory) {
      println!("An error happened: {}", err);
    }
  } else {
    println!("An unknown error happened while reading the file.");
  }
}