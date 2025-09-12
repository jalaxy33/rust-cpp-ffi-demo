# Rust-CPP FFI Demo

A simple demonstration of Foreign Function Interface (FFI) between Rust and C++.

## Overview

This project shows how to call C++ functions from Rust and vice versa using FFI bindings.

## Prerequisites

- Rust (latest stable version)
- C++ compiler (GCC/Clang/MSVC)
- CMake (optional)

## Building

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run the demo
cargo run
```

## Project Structure

```
├── src/
│   ├── lib.rs          # Rust FFI bindings
│   └── main.rs         # Main application
├── cpp/
│   └── math.cpp        # C++ implementation
└── build.rs            # Build script
```

> **Note**: This project structure may change in future updates.

## Usage

The demo showcases basic FFI operations between Rust and C++.

## License

MIT License