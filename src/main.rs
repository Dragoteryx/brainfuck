use clap::Clap;
use std::fs;

pub mod lex; use lex::*;
pub mod memory; use memory::*;
pub mod parse; use parse::*;

// run

fn run<'a, I: Send + Clone + Iterator<Item = &'a Instruction>>(instructions: &mut I, memory: &mut Memory) -> Result<(), String> {
  for instruction in instructions {
    instruction.run(memory)?;
  }
  Ok(())
}

// main

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
  debug: bool
}

fn main() {
  let args = Args::parse();
  if let Ok(content) = fs::read_to_string(&args.file) {
    let tokens = lex(&content);
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