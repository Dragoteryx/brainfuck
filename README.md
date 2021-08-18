# Brainfuck

A Brainfuck interpreter written in Rust.\
I originally made this to learn the language, but am now trying to add as many features as possible.

```
USAGE:
    brainfuck.exe [FLAGS] [OPTIONS] <file>

ARGS:
    <file>    The Brainfuck file to run

FLAGS:
    -d, --debug           Printing the current cell prints debug information
    -h, --help            Prints help information
    -l, --larger-cells    Cells use 4 bytes instead of 1
    -n, --no-overflows    Exit on cell overflows
    -V, --version         Prints version information
    -w, --wrap-around     Wrap around when reaching the leftmost or rightmost cell

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