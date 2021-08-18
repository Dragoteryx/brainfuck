use colored::Colorize;
use clap::Clap;
use std::fs;

mod lex; use lex::*;
mod memory; use memory::*;
mod parse; use parse::*;

// run

fn run<'a, I: Send + Clone + Iterator<Item = &'a Instruction>>(instructions: &mut I, memory: &mut Memory) -> Result<(), String> {
  for instruction in instructions {
    instruction.run(memory)?;
  }
  Ok(())
}

// main

#[derive(Clap, Debug)]
#[clap(version = env!("CARGO_PKG_VERSION"))]
pub struct Args {
  #[clap(about = "The Brainfuck file to run")]
  file: String,

  #[clap(short, long, about = "Set the number of cells in memory", default_value = "30000")]
  size: usize,

  #[clap(short, long, about = "Wrap around when reaching the leftmost or rightmost cell")]
  wrap_around: bool,

  #[clap(short, long, about = "Disable cell overflows")]
  no_overflows: bool,

  #[clap(short, long, about = "Printing the current cell prints debug information")]
  debug: bool
}

fn main() {
  let args = Args::parse();
  if let Ok(content) = fs::read_to_string(&args.file) {
    let tokens = lex(&content);
    match parse(&mut tokens.iter()) {
      Ok(instructions) => {
        let mut memory = Memory::new(&args);
        if let Err(err) = run(&mut instructions.iter(), &mut memory) {
          eprintln!("{} {}", "runtime error:".red(), err);
        }
      }
      Err(err) => {
        eprintln!("{} {}", "compilation error:".red(), err);
      }
    }
  } else {
    eprintln!("{} {}", "error:".red(), "Couldn't read the file, are you sure the path is valid?");
  }
}