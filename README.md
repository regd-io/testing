# regd testing

regd testing is a collection of utility functions and helpers designed to facilitate testing in regd projects. It provides a set of common utilities that enhance test reliability, ease of use, and maintainability.

## Features

- **Randomized Testing Support**: Provides utilities for generating random values.
- **Extensions for Rust Types**: Offers extensions for commonly used Rust types.
- **File I/O Support**: Simplifies test-related file operations.

## Installation

To use `regd-testing` in your Rust project, add the following dependency to your `Cargo.toml`:

```toml
[dev-dependencies]
regd-testing = "0.1.0"
```

## Usage

```rust
use regd_testing::rand;

fn main() {
    let x: u32 = rand::generate();
    println!("Generated random value: {}", x);
}
```

## License

This project is licensed under the **Apache 2.0 License**.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.
