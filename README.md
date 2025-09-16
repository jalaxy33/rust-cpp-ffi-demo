# Rust-CPP FFI Demo

A simple demonstration of Foreign Function Interface (FFI) between Rust and C++.

## Overview

This project shows how to call C++ functions from Rust and vice versa using FFI bindings.

## Prerequisites

- [Rust toolchain](https://www.rust-lang.org/tools/install)
- C++ compiler (GCC/Clang/MSVC)
- [CMake](https://cmake.org/download/)

## Building

**1. Build Rust**

```bash
# debug build
cargo b

# release build
cargo b -r
```

**2. Build C++** (Maybe not right)

```bash
# debug build
cmake --build build --target ALL_BUILD

# release build
cmake --build build --target ALL_BUILD --config Release
```

## Running

**1. C++ call Rust**
```bash
# windows
./build/Debug/call-rust.exe
./build/Release/call-rust.exe

# unix
./build/Debug/call-rust
./build/Release/call-rust

```

**2. Rust call C++**
```bash
cargo r --bin cpp-to-rust
```
