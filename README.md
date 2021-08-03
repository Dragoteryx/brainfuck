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
    -f, --fork       Enable the use of the Y operator (Brainfork)
    -h, --help       Prints help information
    -V, --version    Prints version information
    -w, --wrap       Wrap around when reaching the leftmost or rightmost cell

OPTIONS:
    -s, --size <size>    Set the number of cells in memory [default: 30000]
```