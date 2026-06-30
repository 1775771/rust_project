# Concurrent Port Scanner

A simple multithreaded TCP port scanner written in Rust.

The program scans a user-defined range of TCP ports on an IPv4 address by spawning one thread per port. Each thread attempts to establish a TCP connection with a one-second timeout and reports whether the port is open or closed.

## Features

- Concurrent scanning using Rust threads
- IPv4 address validation
- Port range validation
- TCP connection timeout
- Summary of:
  - Number of closed ports
  - List of open ports

## Project Structure

```
concurrent_port_scanner/
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

Run the scanner with an IPv4 address and a port range:

```bash
cargo run -- <IP_ADDRESS> <START_PORT>-<END_PORT>
```

Example:

```bash
cargo run -- 127.0.0.1 20-100
```

Example output:

```text
Number of closed ports: 79
Open ports: [22, 80]
```

## How it Works

1. Validates the command-line arguments.
2. Parses the IPv4 address.
3. Parses the port range.
4. Creates one thread for each port.
5. Attempts a TCP connection with a one-second timeout.
6. Collects all thread results.
7. Displays the open ports and the total number of closed ports.

## Limitations

- Supports IPv4 only.
- Scans TCP ports only.
- Creates one thread per port, making it unsuitable for very large port ranges due to resource usage.
- A closed, filtered, or unreachable port is reported simply as closed.

## Future Improvements

- Thread pool instead of one thread per port.
- Configurable timeout.
- IPv6 support.
- Service name detection.
- Output formatting (JSON, CSV).
- Better error reporting.
- Progress indicator.
