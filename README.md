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
regd-testing = "0.1.1"
```

## Usage

<details>
<summary>Generate Random Values</summary>

```rust
use regd_testing::rand;

fn main() {
    let x: u32 = rand::generate();
    println!("Generated random value: {}", x);

    let y: i32 = rand::generate_range(-10..10);
    println!("Generated value in range: {}", y);

    let z: String = rand::generate_alphanumeric(16);
    println!("Generated alphanumeric string: {}", z);
}
```

</details>

<details>
<summary>Shuffle and Choose from a Collection</summary>

```rust
use regd_testing::prelude::*;

fn main() {
    let mut xs = [1, 2, 3, 4, 5];
    xs.shuffle();
    println!("Shuffled: {:?}", xs);

    if let Some(x) = xs.choose() {
        println!("Randomly chosen: {}", x);
    }

    if let Some(x) = xs.choose_mut() {
        *x = 99;
        println!("Modified chosen value: {}", x);
    }
}
```

</details>

<details>
<summary>Create and Read Temporary Files</summary>

```rust
use std::fs;

use regd_testing::io;

fn main() {
    let file = io::try_new_tempfile("temporary content")
        .expect("should succeed in creating a temporary file");

    let path = file.path();
    println!("Temp file path: {:?}", path);

    let contents = fs::read_to_string(path)
        .expect("should read file content");
    assert_eq!(contents, "temporary content");
}
```

</details>

<details>
<summary>Create, Read, and Remove Named Files</summary>

```rust
use std::fs;
use std::io::{self, Read};

use regd_testing::io as regd_testing_io;

fn main() {
    let file = regd_testing_io::try_new_file("test.txt", "hello world")
        .expect("should be able to create file");

    let mut contents = String::new();
    io::BufReader::new(file)
        .read_to_string(&mut contents)
        .expect("should be able to read the file");
    assert_eq!(contents, "hello world");
    println!("Read from file: {}", contents);

    io::try_remove_file("test.txt")
        .expect("should remove file successfully");

    let bad_filename = regd_testing::rand::generate_badfile(12);
    assert!(fs::metadata(&bad_filename).is_err());
    println!("Generated fake filename: {}", bad_filename);
}
```

</details>

## License

This project is licensed under the **Apache 2.0 License**.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.
