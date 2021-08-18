# Brainfuck

A Brainfuck interpreter written in Rust.\
I made this to learn the language.

```
USAGE:
    brainfuck.exe [FLAGS] [OPTIONS] <file>

ARGS:
    <file>    The Brainfuck file to run

FLAGS:
    -d, --debug           Printing the current cell prints debug information
    -h, --help            Prints help information
    -n, --no-overflows    Disable cell overflows
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