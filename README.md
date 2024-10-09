filemagic-rs
-------------
![Crates.io](https://img.shields.io/crates/v/filemagic)
[![Documentation](https://docs.rs/filemagic/badge.svg)](https://docs.rs/filemagic)
[![Build Status](https://travis-ci.org/marirs/filemagic-rs.svg?branch=master)](https://travis-ci.org/marirs/filemagic-rs)

filemagic is a [Rust](http://www.rust-lang.org/) wrapper for [libmagic](http://darwinsys.com/file/), the library that supports the file command on most Unix systems. 
The package provides a simple [Rust](http://www.rust-lang.org/) API for identifying files using the extensive database of magic strings that ships with libmagic.
It can also load a custom database of magic strings.

### Requirements
- `Rust 1.40.0` or above stable version
- `libmagic` 
  - macOS: `brew install libmagic`
  - Linux: `apt install libmagic1 libmagic-dev`  

### Usage

Adding dependency to your `Cargo.toml` file
```toml
filemagic = "0.12.5"
```

## vendored

The `vendored` feature uses the [`cc` crate](https:/docs.rs/cc) to compile and
static link a vendored version of libmagic, currently based on 5.45.

Adding dependency to your `Cargo.toml` file
```toml
filemagic = { version = "0.12.5", features = ["vendored"] }
```

---
### Using Macros

- Using default libmagic database:
```rust
use filemagic::magic;

fn main() {
  let test_file_path = "/path/to/file";
  let magic = magic!().expect("error");
  
  println!("{}", magic.file(&test_file_path).expect("error"));
}
```

- Using custom Magic database:
```rust
use filemagic::magic;

fn main() {
  let test_file_path = "/path/to/file";
  let databases = vec!["data/db-images-png"];
  
  let magic = magic!(,&databases).expect("error");
  
  println!("{}", magic.file(&test_file_path).expect("error"));
}
```

---
### Using the function

- Using the default libmagic database:
```rust
use filemagic::Magic;

fn main() {
    let test_file_path = "/path/to/file";
    // Create a new default configuration
    let cookie = Magic::open(Default::default()).expect("error");
    cookie.load::<String>(&[]).expect("error");
    let magic = cookie.file(&test_file_path).expect("error in magic");
    println!("magic= {}", magic);
}
```

- Using custom Magic database:
```rust
use filemagic::Magic;

fn main() {
    // Create a new default configuration
    let cookie = Magic::open(Default::default()).expect("error");
    // Load one specific magic database
    let databases = vec!["data/db-images-png"];
    assert!(cookie.load(&databases).is_ok());

    // Recognize the magic of a test file
    let test_file_path = "data/rust-logo-128x128-blk.png";
    let expected_magic = "PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced";
    assert_eq!(cookie.file(&test_file_path).unwrap(), expected_magic);
}
```

---
### To generate the docs
```bash
cargo doc --release
```

---
References:
- robo9k [rust-magic](https://github.com/robo9k/rust-magic) & [rust-magic-sys](https://github.com/robo9k/rust-magic-sys)
- [Aaron Iles](https://github.com/aliles/filemagic)
