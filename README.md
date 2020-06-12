# configparser
[![Build Status](https://travis-ci.com/QEDK/configparser-rs.svg?branch=master)](https://travis-ci.com/QEDK/configparser-rs) [![Crates.io](https://img.shields.io/crates/l/configparser?color=black)](LICENSE-MIT) [![Crates.io](https://img.shields.io/crates/v/configparser?color=black)](https://crates.io/crates/configparser) [![Released API docs](https://docs.rs/configparser/badge.svg)](https://docs.rs/configparser) [![Maintenance](https://img.shields.io/maintenance/yes/2020)](https://github.com/QEDK/configparser-rs)

A simple configuration parsing utility with no dependencies built on Rust.

`configparser` works on a subset of ini configuration syntax (for now). It is inspired by Python's `configparser`.

The current release is experimental, this means that future releases will be swift until we reach `stable` (1.0.0).
The codebase is thus subject to change for now.

## ini-style configuration

Most `ini` files look something like this (but they don't need to be `.ini` files obviously):
```yaml
[some-section]
key1 = value1
key2 = value2

[some-other-section]
key3 = value3
maybekey1aswell = value1
```
Owing to how ini files usually are, this means that `[`, `]` and `=` are special symbols (this crate will allow you to use `]` sparingly).

Key-value pairs or section headers cannot spread across multiple lines for obvious reasons because the parser cannot reliably parse it otherwise.
A value on the next line could as well be a key for another.

An important note is that key-value pairs not attached to any section are automatically put in a section called `DEFAULT`.
Future releases will add support for escaping, comments and modifying default section naming.


## Installation
You can install this easily via `cargo` by including it in your `Cargo.toml` file like:
```yaml
[dependencies]
configparser = "0.3.0"
```

## Usage
You can load an `ini`-file easily and parse it like:
```rust
use configparser::ini::Ini;
use std::collections::HashMap;

fn main() {
  let mut config = Ini::new();

  match config.load("Path/to/file...") {
  	Err(why) => panic!("{}", why),
  	Ok(_) => println!("Yay!")
  }

  // You can then access the map normally like:
  let map = match config.get_map() {
  	None => HashMap::new(), // or whatever you want to do if the map is empty
  	Some(map) => map
  }; // or let map = config.get_map().unwrap() instead of match

  for (key, value) in &map {
      println!("{:?}: {:?}", key, value);
  }
  // And safely fetch a value:
  let val = config.get("section name", "key name").unwrap();
}
```
The `Ini` struct is the way to go forward and will soon have more features, such as reading from a string, insertion, deletion and variable access.

As of now, there's also a public function, to load an ini-syntax file and parse it into a hashmap. Support for this will be dropped in the near future, and will be changed into a macro when it's dropped.
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

## License

Licensed under either of

 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
 * Lesser General Public license v3.0 ([LICENSE-LGPL](LICENSE-LGPL) or https://www.gnu.org/licenses/lgpl-3.0.html)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the LGPL-3.0 license, shall be dual licensed as above, without any
additional terms or conditions.

## Changelog

- 0.1.0 (yanked)
  - First experimental version with only a public-facing load() function.
- 0.1.1
  - `configparser` module renamed to `ini`.
- 0.2.1
  - `Ini` struct is added along with file-loading, parsing and hashmap functions. Documentation is added.
- 0.2.2
  - Fixed docs.
- 0.3.0
  - Added `get()` for getting values from the map directly. Docs expanded as well.
  - Mark `ini::load()` for deprecation.

### Future plans

- Support for `ini::load()` will be dropped in the next major releaser per SemVer (i.e. 1.0.0)
  - It will be replaced with a macro for a similar functionality.
  - It has been marked as deprecated.
- More functions for `Ini` struct, such as reading from a string, insertion, deletion.
