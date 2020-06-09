# configparser
A simple configuration parsing utility with no dependencies built on Rust.

`configparser` works on a subset of ini configuration syntax.

Inspired by Python's `configparser`. This release is experimental, use at your own risk.

## Installation
You can install this easily via `cargo` like:
```bash
$ cargo install configparser
```

## Usage
As of now, there's only one function, to load an ini-syntax file and parse it into a hashmap of type `HashMap<String, HashMap<String, String>>`.
```rust
use configparser::ini;

fn main() {
  let map = ini::load("Path/to/file...");

  // You can then access the map normally like:
  for (key, value) in &map {
      println!("{:?}: {:?}", key, value);
  }
  // ...and do what you want with it. :)
}
```
