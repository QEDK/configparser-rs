# configparser
[![Build Status](https://travis-ci.com/QEDK/configparser-rs.svg?branch=master)](https://travis-ci.com/QEDK/configparser-rs) [![Crates.io](https://img.shields.io/crates/l/configparser?color=black)](LICENSE-MIT) [![Crates.io](https://img.shields.io/crates/v/configparser?color=black)](https://crates.io/crates/configparser) [![Released API docs](https://docs.rs/configparser/badge.svg)](https://docs.rs/configparser) [![Maintenance](https://img.shields.io/maintenance/yes/2020)](https://github.com/QEDK/configparser-rs)

This crate provides the `Ini` struct which implements a basic configuration language which provides a structure similar to what’s found in Windows' `ini` files. You can use this to write Rust programs which can be customized by end users easily.

This is a simple configuration parsing utility with no dependencies built on Rust. It is inspired by Python's `configparser`.

The current release is experimental, this means that future releases will be swift until we reach `stable` (1.0.0).
The codebase is thus subject to change for now.

## Quick Start

A basic `ini`-syntax file (we say ini-syntax files because the files don't need to be necessarily `*.ini`) looks like this:
```INI
[DEFAULT]
key1 = value1
pizzatime = yes
cost = 9

[topsecrets]
nuclear launch codes = topsecret

[github.com]
User = QEDK
```
Essentially, the syntax consists of sections, each of which can which contains keys with values. The `Ini` struct can read and write such values.

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
values like this= 0000
or this= 0.999
are they treated as numbers? = no
integers, floats and booleans are held as= strings

[value-less?]
a_valueless_key_has_None
this key has an empty string value has Some("") =

    [indented sections]
        can_values_be_as_well = True
        purpose = formatting for readability
        is_this_same     =        yes
            is_this_same=yes
```
An important thing to note is that values with the same keys will get updated, this means that the last inserted key (whether that's a section header
or property key) is the one that remains in the `HashMap`.

## Installation
You can install this easily via `cargo` by including it in your `Cargo.toml` file like:
```TOML
[dependencies]
configparser = "0.6.0"
```

## Usage
You can load an `ini`-file easily and parse it like:
```rust
use configparser::ini::Ini;
use std::collections::HashMap;

fn main() {
  let mut config = Ini::new();

  // You can easily load a file to get a clone of the map:
  let map = match config.load("tests/test.ini") {
  	Err(why) => panic!("{}", why),
  	Ok(map) => map
  }; // You can also safely not store the reference and access it later with get_map_ref()
  // or get a clone with get_map()

  // You can then access it like a normal hashmap:
  let innermap = map["topsecret"]; // Remember this is a hashmap!

  // If you want to access the value, then you can simply do:
  let val = map["topsecret"]["KFC"].clone().unwrap();
  // Remember that the .clone().unwrap() is required because it's an Option<String> type!

  // What if you want to mutate the parser and remove KFC's secret recipe? Just use get_mut_map()
  let mut_map = config.get_mut_map();
  mut_map.get_mut("topsecret").unwrap().insert(String::from("KFC"), None);
  // And the secret is back in safety, remember that these are normal HashMap functions chained for convenience.

  // However very quickly see how that becomes cumbersome, so you can use the handy get() function from Ini
  let val = config.get("topsecret", "KFC"); // unwrapping will be an error because we just emptied it!

  assert_eq!(val, None);
}
```
The `Ini` struct is the way to go forward and will soon have more features, such as reading from a string, insertion, deletion, index access
as well as support for comments.

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
- 0.5.0 (**BETA**) (yanked)
  - Changelog added.
  - Support for value-less keys.
  - `HashMap` values are now `Option<String>` instead of `String` to denote empty values vs. no values.
  - Documentation greatly improved.
  - Syntax docs provided.
  - `new()` and `get()` methods are simplified.
- 0.5.1
  - Fixed erroneous docs.
- 0.6.0 (**BETA 2**)
  - `Ini::load` now gives an immutable reference to the map.
  - `get_map_ref()` and `get_mut_map()` are now added to allow direct `HashMap` index access making things greatly easier.

### Future plans

- Support for `ini::load()` will be dropped in the next major releaser per SemVer (i.e. 1.0.0)
  - It will be replaced with a macro for a similar functionality.
  - It has been marked as deprecated.
- More functions for `Ini` struct, such as reading from a string, insertion and deletion.
- Support for comments
- Support for value-parsing
