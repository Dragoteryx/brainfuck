use std::io::{stdout, Write};
use std::num::NonZeroU32;
use std::time::Instant;
use colored::Colorize;
use clap::Clap;
use std::fs;

mod error; use error::*;
mod instruction; use instruction::*;
mod memory; use memory::*;
mod optimise; use optimise::*;
mod token; use token::*;

// run

fn run<T>(instructions: &Vec<Instruction>, memory: &mut impl Memory<T>) -> Result<(), Error> {
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

  #[clap(short, long, about = "Set the size of cells in bits", default_value = "8", possible_values = &["8", "16", "32"])]
  cell_size: String,

  #[clap(short, long, about = "Set the number of cells in memory", default_value = "30000")]
  memory_size: NonZeroU32,

  #[clap(short, long, about = "Wrap around when reaching the leftmost or rightmost cell")]
  wrap_around: bool,

  #[clap(short, long, about = "Exit on cell overflows")]
  no_overflows: bool,

  #[clap(short, long, about = "Disables all optimisations")]
  unoptimised: bool,

  #[clap(short, long, about = "Prints how long the program took to execute")]
  timed: bool,

  #[clap(short, long, about = "Printing the current cell prints debug information")]
  debug: bool
}

#[allow(unused_must_use)]
fn main() {
  let args = Args::parse();
  if let Ok(content) = fs::read_to_string(&args.file) {
    let tokens = lex(&content);
    match parse(&tokens) {
      Ok(mut instructions) => {
        if !args.unoptimised {
          instructions = optimise(instructions);
        }
        let now = Instant::now();
        if args.cell_size == "8" {
          let mut memory = Memory8::new(&args);
          if let Err(err) = run(&instructions, &mut memory) {
            stdout().flush();
            eprintln!("\n{} {}", "runtime error:".red(), err.message());
          } else if args.timed {
            stdout().flush();
            let duration = Instant::now() - now;
            println!("\n{} {}s", "duration:".green(), duration.as_secs_f64());
          }
        } else if args.cell_size == "16" {
          let mut memory = Memory16::new(&args);
          if let Err(err) = run(&instructions, &mut memory) {
            stdout().flush();
            eprintln!("\n{} {}", "runtime error:".red(), err.message());
          } else if args.timed {
            stdout().flush();
            let duration = Instant::now() - now;
            println!("\n{} {}s", "duration:".green(), duration.as_secs_f64());
          }
        } else if args.cell_size == "32" {
          let mut memory = Memory32::new(&args);
          if let Err(err) = run(&instructions, &mut memory) {
            stdout().flush();
            eprintln!("\n{} {}", "runtime error:".red(), err.message());
          } else if args.timed {
            stdout().flush();
            let duration = Instant::now() - now;
            println!("\n{} {}s", "duration:".green(), duration.as_secs_f64());
          }
        }
      }
      Err(err) => {
        eprintln!("{} {}", "compilation error:".red(), err.message());
      }
    }
  } else {
    eprintln!("{} {}", "error:".red(), "Couldn't read the file, are you sure the path is valid?");
  }
}