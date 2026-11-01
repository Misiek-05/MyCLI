# my_cli

`my_cli` is a small command-line utility written in Rust. It provides a collection of familiar file and directory commands, along with a welcome message and a text calendar. It is implemented as a learning project using Rust enums, modules, filesystem APIs, and the `clap` command-line parser.

## Features

- Print a welcome message, optionally repeating `HI` a chosen number of times.
- Create files and directories.
- Display file contents, or a chosen number of words from a file.
- List the current directory, including file sizes, attributes, and modification times.
- Print the current working directory and copy its path to the system clipboard.
- Copy, move, rename, and remove files; remove empty directories.
- Show the first or last lines of a file.
- Print text to the terminal or write it to a file.
- Display the current month or the full current year as a calendar.

## Technologies

- **Rust 2024 edition**
- **clap** with derive macros for parsing subcommands and options
- **chrono** for local date and calendar calculations
- **colored** for colored terminal output
- **crossterm** for terminal cursor positioning
- **arboard** for copying the working directory path to the clipboard
- **indicatif** is included as a dependency but is not currently used in the source code

The current implementation uses Windows-specific filesystem metadata APIs, so it is intended to be built and run on Windows.

## Requirements

- Rust and Cargo (installable through [rustup](https://rustup.rs/))
- Windows

## Build and run

From the project directory, build the application with:

```bash
cargo build
```

Run a command through Cargo with:

```bash
cargo run -- <command> [arguments]
```

For example:

```bash
cargo run -- welcome
cargo run -- welcome --count 3
```

After building, the executable is available under `target/debug/` (`my_cli.exe` on Windows). Run `my_cli --help` to list commands, or `my_cli <command> --help` to see a command's options.

## Commands

| Command | Description | Example |
| --- | --- | --- |
| `welcome` | Print a welcome message; `--count` repeats `HI` | `my_cli welcome --count 3` |
| `touch <name>` | Create a file | `my_cli touch notes.txt` |
| `cat <name>` | Print a file; `--number` limits output to a number of words | `my_cli cat notes.txt --number 10` |
| `pwd` | Print the current directory and copy it to the clipboard | `my_cli pwd` |
| `ls` | List entries in the current directory | `my_cli ls` |
| `cp <source-name> <dest-name>` | Copy a file | `my_cli cp notes.txt backup.txt` |
| `mv <source-name> <dest-name>` | Move or rename a file | `my_cli mv notes.txt archive/notes.txt` |
| `rm <name>` | Remove a file or an empty directory | `my_cli rm notes.txt` |
| `mkdir <name>` | Create a directory, including missing parent directories | `my_cli mkdir archive/2026` |
| `head <name> --count <n>` | Print the first *n* lines (10 by default) | `my_cli head notes.txt --count 5` |
| `tail <name> --count <n>` | Print the last *n* lines (10 by default) | `my_cli tail notes.txt --count 5` |
| `echo <text> [--name <file>]` | Print text, or write it to the named file | `my_cli echo "Hello" --name greeting.txt` |
| `cal [--year]` | Print the current month, or the full current year | `my_cli cal --year` |

## Project structure

```text
my_cli/
|-- Cargo.toml
|-- Cargo.lock
`-- src/
    |-- main.rs       Command dispatch and application entry point
    |-- cli_struct.rs clap CLI definition
    |-- enums_file.rs Subcommand definitions and help text
    |-- subcommands.rs Command implementations
    `-- functions.rs  Clipboard and calendar helpers
```
