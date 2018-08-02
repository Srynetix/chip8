chip8
=====

![screen](assets/screen.gif)

CHIP-8 emulator written in Rust.

Uses rust-sdl2 for windowing and rendering.

Features
--------

- Support almost all CHIP-8 games
- Game disassembler
- Real-time debugger
- Save-states system

Command-line help
-----------------

```bash
USAGE:
    chip8 [FLAGS] [OPTIONS] <FILENAME>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    verbose mode

OPTIONS:
    -b, --breakpoint <breakpoint>...    add breakpoint at address
    -d, --disassemble <disassemble>     disassemble cartridge to file (use '-' to trace in console)
    -t, --trace <trace>                 trace execution to file

ARGS:
    <FILENAME>    cartridge name (not the path)
```