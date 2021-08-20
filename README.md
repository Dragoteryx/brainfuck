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
    -n, --no-overflows    Exit on cell overflows
    -t, --timed           Prints how long the program took to execute
    -u, --unoptimised     Disables all optimisations
    -V, --version         Prints version information
    -w, --wrap-around     Wrap around when reaching the leftmost or rightmost cell

OPTIONS:
    -c, --cell-size <cell-size>        Set the size of cells in bits [default: 8] [possible values: 8, 16, 32]
    -m, --memory-size <memory-size>    Set the number of cells in memory [default: 30000]
```

## Build

Install the latest version of the Rust compiler.\
Download the source files.\
Build the files using this command:
```
cargo build --release
```
If everything went well, the executable should be in `/target/release`.