# Minimal ELF Header Analysis

A lightweight ELF header reader written in Rust.

The program reads the first 16 bytes (the `e_ident` field) of a file, verifies that it is an ELF executable, and displays basic information such as the architecture class, endianness, and target ABI.

## Features

- Validates command-line arguments
- Checks that the provided path exists and is a file
- Reads the ELF identification header (`e_ident`)
- Detects whether the file is a valid ELF binary
- Displays:
  - ELF class (32-bit or 64-bit)
  - Endianness
  - ABI
  - ABI version

## Project Structure

```
minimal_elf_header_analysis/
├── Cargo.toml
└── main.rs
```

## Requirements

- Rust (edition 2024)
- Cargo

## Build

```bash
cargo build --release
```

## Usage

Run the program with the path to an ELF file:

```bash
cargo run -- <FILE>
```

Example:

```bash
cargo run -- /bin/ls
```

Example output:

```text
"/bin/ls" is a file
"/bin/ls" is an ELF file.
Class       : ELF 64-bit
Endianness  : Little endian
ABI         : System V
ABI version : 0
```

## How it Works

1. Validates the command-line arguments.
2. Ensures the provided path exists and is a regular file.
3. Reads the first 16 bytes of the file.
4. Checks the ELF magic number (`0x7F 45 4C 46`).
5. Parses the `e_ident` structure.
6. Displays basic information extracted from the ELF header.

## Limitations

- Reads only the ELF identification header (`e_ident`).
- Supports only the ABI values implemented in the source code.
- Does not parse the complete ELF header, program headers, or section headers.
- Does not inspect symbols, segments, or sections.

## Future Improvements

- Parse the complete ELF header.
- Display entry point address.
- Show machine architecture (x86, x86-64, ARM, etc.).
- Parse program headers.
- Parse section headers.
- Display symbols and string tables.
- Support more ABI values and ELF versions.
