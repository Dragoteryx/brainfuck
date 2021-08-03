use clap::Clap;
use std::fs;

pub mod lex; use lex::*;
pub mod memory; use memory::*;
pub mod parse; use parse::*;

// run

fn run<'a, I: Iterator<Item = &'a Instruction>>(instructions: &mut I, memory: &mut Memory) -> Result<(), String> {
  loop {
    if let Some(instruction) = instructions.next() {
      //println!("instruction: {:?}", instruction);
      match instruction {
        Instruction::Increment => Ok(memory.increment()),
        Instruction::Decrement => Ok(memory.decrement()),
        Instruction::MoveRight => memory.move_right(),
        Instruction::MoveLeft => memory.move_left(),
        Instruction::Write => memory.write(),
        Instruction::Read => memory.read(),
        Instruction::Loop(inner) => {
          while memory.get_value() != 0 {
            run(&mut inner.iter(), memory)?;
          }
          Ok(())
        }
      }?;
    } else {
      break;
    }
  }
  Ok(())
}

// cli

#[derive(Clap)]
#[clap(version = env!("CARGO_PKG_VERSION"))]
struct Args {
  #[clap(about = "The Brainfuck file to run")]
  file: String,

  #[clap(short, long, about = "Set the number of cells in memory", default_value = "30000")]
  size: usize,

  #[clap(short, long, about = "Enable wrap around")]
  wrap: bool
}

fn main() {
  let arg = Args::parse();
  if let Ok(content) = fs::read_to_string(&arg.file) {
    let tokens = lex(&content);
    match parse(&mut tokens.iter()) {
      Ok(instructions) => {
        let mut memory = Memory::new(arg.size, arg.wrap);
        if let Err(err) = run(&mut instructions.iter(), &mut memory) {
          eprintln!("Runtime error: {}", err);
        }
      }
      Err(err) => {
        eprintln!("Compile time error: {}", err);
      }
    }
  } else {
    eprintln!("Couldn't read the file. Are you sure the path is valid?");
  }
}