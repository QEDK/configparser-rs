# configparser
[![Build Status](https://travis-ci.com/QEDK/configparser-rs.svg?branch=master)](https://travis-ci.com/QEDK/configparser-rs) [![Crates.io](https://img.shields.io/crates/l/configparser?color=black)](LICENSE-MIT) [![Crates.io](https://img.shields.io/crates/v/configparser?color=black)](https://crates.io/crates/configparser) [![Released API docs](https://docs.rs/configparser/badge.svg)](https://docs.rs/configparser) [![Maintenance](https://img.shields.io/maintenance/yes/2020)](https://github.com/QEDK/configparser-rs)


This crate provides the `Ini` struct which implements a basic configuration language which provides a structure similar to what’s found in Windows' `ini` files. You can use this to write Rust programs which can be customized by end users easily.

This is a simple configuration parsing utility with no dependencies built on Rust. It is inspired by Python's `configparser`.

The current release is experimental, this means that future releases will be swift until we reach `stable` (1.0.0).
The codebase is thus subject to change for now.

## Quick Start

A basic `ini`-syntax (we say ini-syntax files because the files don't need to be necessarily `*.ini`) file looks like this:
```INI
[DEFAULT]
key1 = value1
pizzatime = yes
cost = 9

[github.com]
User = QEDK

[topsecrets]
API_KEY = topsecret
```
Essentially, the syntax consists of sections, each of can which contains keys with values. The `Ini` struct can read and write such values.

## Supported datatypes
`configparser` does not guess the datatype of values in configuration files and stores everything as strings. If you need other datatypes, you should
parse them yourself. It's planned to implement getters for primitive datatypes in the future.
```rust
let my_int = my_string.parse::<i32>().unwrap();
let my_str = my_string.as_str();
```

## Supported `ini` file structure
A configuration file can consist of sections, each led by a `[section-name]` header, followed by key-value entries separated by a `=`. By default, section names and key names are case-insensitive. All leading and trailing whitespace is removed from stored keys, values and section names.
Key values can be omitted, in which case the key-value delimiter (`=`) may also be left out (but this is different from putting a delimiter, we'll
explain it later). Key-value pairs or section headers cannot span multiple lines.
Owing to how ini files usually are, this means that `[`, `]` and `=` are special symbols (this crate will allow you to use `]` sparingly).

Let's take for example:
```INI
[Basic Values is the same thing]
[   Basic Values is the same thing    ]
key1=value1
spaces in keys=allowed
spaces in values=allowed as well
spaces around the delimiter = also OK

[All values are strings]
values like this: 0000
or this: 0.999
are they treated as numbers? : no
integers, floats and booleans are held as: strings

[value-less?]
a_valueless_key
this key has an empty string value =

    [indented sections]
        can_values_be_as_well = True
        purpose = formatting for readability
        is_this_same     =        yes
            is_this_same=yes
```
An important thing to note is that values with the same keys will get updated, this means that the last inserted value is the one that remains
in the `HashMap`.

## Installation
You can install this easily via `cargo` by including it in your `Cargo.toml` file like:
```yaml
[dependencies]
configparser = "0.4.1"
```

## Usage
You can load an `ini`-file easily and parse it like:
```rust
use configparser::ini::Ini;
use std::collections::HashMap;

fn main() {
  let mut config = Ini::new();

  let a_map_clone = match config.load("Path/to/file...") {
  	Err(why) => panic!("{}", why),
  	Ok(map) => map
  }; // You can also safely not store the HashMap and access it later

  // You can also then access the map normally like:
  let another_clone = match config.get_map() {
  	None => HashMap::new(), // or whatever you want to do if the map is empty
  	Some(map) => map
  }; // or let map = config.get_map().unwrap() instead of match

  for (key, value) in &another_clone {
      println!("{:?}: {:?}", key, value);
  }
  // And safely fetch a value:
  let val = config.get("section name", "key name").unwrap();
}
```
The `Ini` struct is the way to go forward and will soon have more features, such as reading from a string, insertion, deletion and variable access.

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

Old changelogs are in [CHANGELOG.md](CHANGELOG.md).
- 0.3.0
  - Added `get()` for getting values from the map directly. Docs expanded as well.
  - Mark `ini::load()` for deprecation.
- 0.3.1
  - Updated docs.
  - All parameters now trimmed before insertion.
  - Converted `ini::load()` into a wrapper around `Ini`.
- 0.4.0
  - Changed `Ini::load()` to return an `Ok(map)` with a clone of the stored `HashMap`.
- 0.4.1
  - Fixed and added docs.
- 0.5.0 (**BETA**)
  - Changelog added.
  - Support for value-less keys.
  - `HashMap` values are now `Option<String>` instead of `String` to denote empty values vs. no values.
  - Documentation greatly improved.
  - Syntax docs provided.
  - `new()` and `get()` methods are simplified.

### Future plans

- Support for `ini::load()` will be dropped in the next major releaser per SemVer (i.e. 1.0.0)
  - It will be replaced with a macro for a similar functionality.
  - It has been marked as deprecated.
- More functions for `Ini` struct, such as reading from a string, insertion and deletion.
- Support for comments
- Support for value-parsing
- Index access
