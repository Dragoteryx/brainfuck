use std::sync::mpsc::channel;
use std::{fs, thread};
use clap::Clap;

pub mod lex; use lex::*;
pub mod memory; use memory::*;
pub mod parse; use parse::*;

// run

fn run<'a, I: Send + Clone + Iterator<Item = &'a Instruction>>(instructions: &mut I, memory: &mut Memory) -> Result<(), String> {
  let mut forks = 0u128;
  let (sender, receiver) = channel();
  while let Some(instruction) = instructions.next() {
    match instruction {
      Instruction::Increment => Ok(memory.increment()),
      Instruction::Decrement => Ok(memory.decrement()),
      Instruction::MoveRight => memory.move_right(),
      Instruction::MoveLeft => memory.move_left(),
      Instruction::Write => memory.write(),
      Instruction::Read => memory.read(),
      Instruction::Fork => {
        let instructions_clone: Vec<Instruction> = instructions.clone().cloned().collect();
        let mut memory_clone = memory.clone();
        let sender_clone = sender.clone();
        forks += 1;
        thread::spawn(move || {
          if let Err(err) = memory_clone.move_right() {
            eprintln!("Runtime error in forked thread: {}", err);
          } else {
            memory_clone.set_value(1);
            if let Err(err) = run(&mut instructions_clone.iter(), &mut memory_clone) {
              eprintln!("Runtime error in forked thread: {}", err);
            }
          }
          sender_clone.send(()).unwrap();
        });
        memory.set_value(0);
        Ok(())
      }
      Instruction::Loop(inner) => {
        while memory.get_value() != 0 {
          run(&mut inner.iter(), memory)?;
        }
        Ok(())
      }
    }?;
  }
  while forks > 0 {
    receiver.recv().unwrap();
    forks -= 1;
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

  #[clap(short, long, about = "Wrap around when reaching the leftmost or rightmost cell")]
  wrap: bool,

  #[clap(short, long, about = "Print information about the current cell instead of the corresponding character")]
  debug: bool,

  #[clap(short, long, about = "Enable the use of the Y operator (Brainfork)")]
  fork: bool
}

fn main() {
  let args = Args::parse();
  if let Ok(content) = fs::read_to_string(&args.file) {
    let tokens = lex(&content, args.fork);
    match parse(&mut tokens.iter()) {
      Ok(instructions) => {
        let mut memory = Memory::new(args.size, args.wrap, args.debug);
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