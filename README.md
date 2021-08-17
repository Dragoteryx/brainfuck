# Brainfuck

A Brainfuck interpreter written in Rust.\
I made this to learn the language.

```
USAGE:
    brainfuck.exe [FLAGS] [OPTIONS] <file>

ARGS:
    <file>    The Brainfuck file to run

FLAGS:
    -d, --debug      Print the values in cells instead of the corresponding character
    -h, --help       Prints help information
    -V, --version    Prints version information
    -w, --wrap       Wrap around when reaching the leftmost or rightmost cell

OPTIONS:
    -s, --size <size>    Set the number of cells in memory [default: 30000]
```

## Build

Install the latest version of the Rust compiler.\
Download the source files.\
Build the files using this command:
```
cargo build --release
```
If everything went well, the executable should be in `/target/release`.