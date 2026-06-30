# Rust Projects

This repository contains a collection of small projects written in Rust. Each project focuses on a specific concept, ranging from networking to binary file analysis, with an emphasis on learning Rust's standard library and systems programming features.

## Projects

### Concurrent Port Scanner

A multithreaded TCP port scanner that scans a user-defined range of ports on an IPv4 address.

**Features**
- Concurrent scanning using one thread per port
- IPv4 address validation
- Port range validation
- TCP connection with timeout
- Lists open ports and reports the number of closed ports

**Location**
```
concurrent_port_scanner/
```

---

### Minimal ELF Header Analysis

A lightweight parser that reads the ELF identification header (`e_ident`) of a binary and extracts basic information.

**Features**
- Validates that the input is a file
- Detects whether the file is a valid ELF binary
- Displays:
  - ELF class (32-bit or 64-bit)
  - Endianness
  - ABI
  - ABI version

**Location**
```
minimal_elf_header_analysis/
```

## Requirements

- Rust (Edition 2024)
- Cargo

## Building a Project

Each project is independent. Navigate to its directory and build it with Cargo:

```bash
cd <project_directory>
cargo build --release
```

or run it directly:

```bash
cargo run --
```

## Repository Structure

```
rust_project/
├── concurrent_port_scanner/
├── minimal_elf_header_analysis/
└── README.md
```
