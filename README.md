# eze

A small, `ls`-like file system representer written in Rust.

`eze` lists the contents of a directory with colored output, Unix-style permission strings, ownership, size, and modification dates — much like the classic `ls` command.

## Features

- 📂 Lists files and directories in a given path (defaults to the current directory)
- 🎨 Colored output — directories are shown in **blue/bold**, files in *italic*
- 🔐 Unix-style permission strings (e.g. `drwxr-xr-x`)
- 👤 Shows file owner and group
- 📏 File size and number of hard links
- 🕒 Human-readable modification date (`%b %d %H:%M`)
- 🙈 Hidden files (dot-files) are skipped unless `--all` is passed

## Installation

Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.

```bash
# Clone the repository
git clone https://github.com/Behruz-s-organization/ls_in_rust.git
cd ls_in_rust

# Build a release binary
cargo build --release

# The binary will be at target/release/eze
./target/release/eze
```

Or install it locally with Cargo:

```bash
cargo install --path .
```

## Usage

```bash
eze [OPTIONS] [FILE_NAME]
```

### Arguments

| Argument      | Description                                          |
| ------------- | ---------------------------------------------------- |
| `FILE_NAME`   | Directory to list. Defaults to `.` (current dir).    |

### Options

| Flag              | Description                                         |
| ----------------- | -------------------------------------------------- |
| `-a`, `--all`     | Include hidden files (entries starting with `.`).  |
| `-l`, `--list`    | Long listing format (permissions, owner, size, …). |
| `-h`, `--help`    | Print help.                                        |
| `-V`, `--version` | Print version.                                     |

### Examples

```bash
# List the current directory (compact)
eze

# List a specific directory
eze src

# Long format
eze -l

# Long format, including hidden files
eze -la
```

Example long-format output:

```
drwxr-xr-x 3 behruz   behruz     4096 Jun 01 12:30 src/
-rw-r--r-- 1 behruz   behruz      198 Jun 01 12:31 Cargo.toml
```

## Project structure

```
src/
├── main.rs     # CLI parsing, directory traversal, output formatting
└── decode.rs   # Helpers: permission-bit decoding and date formatting
```

## Built with

- [clap](https://crates.io/crates/clap) — command-line argument parsing
- [owo-colors](https://crates.io/crates/owo-colors) — terminal coloring
- [chrono](https://crates.io/crates/chrono) — date/time formatting
- [uzers](https://crates.io/crates/uzers) — resolving user and group names

## Platform support

`eze` relies on Unix file metadata (permission modes, uid/gid, hard links) via `std::os::unix`, so it targets **Linux and other Unix-like systems**. It will not build on Windows.

## License

Licensed under the [MIT License](LICENSE).
