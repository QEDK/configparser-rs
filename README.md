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

### Installation
You can install this easily via `cargo` by including it in your `Cargo.toml` file like:
```TOML
[dependencies]
configparser = "0.9.2"
```

## Supported datatypes
`configparser` does not guess the datatype of values in configuration files and stores everything as strings. However, some datatypes are so common
that it's a safe bet that some values need to be parsed in other types. For this, the `Ini` struct provides easy functions like `getint()`, `getuint()`,
`getfloat()` and `getbool()`. The only bit of extra magic involved is that the `getbool()` function will treat boolean values case-insensitively (so
`true` is the same as `True` just like `TRUE`). The crate also provides a stronger `getboolcoerce()` function that parses more values (such as `T`, `yes` and `0`, all case-insensitively), the function's documentation will give you the exact details.
```rust
use configparser::ini::Ini;

let mut config = Ini::new();
config.read(String::from(
  "[somesection]
  someintvalue = 5"));
let my_value = config.getint("somesection", "someintvalue").unwrap().unwrap();
assert_eq!(my_value, 5); // value accessible!

//You can ofcourse just choose to parse the values yourself:
let my_string = String::from("1984");
let my_int = my_string.parse::<i32>().unwrap();
```

## Supported `ini` file structure
A configuration file can consist of sections, each led by a `[section-name]` header, followed by key-value entries separated by a `=`. By default, section names and key names are case-insensitive. All leading and trailing whitespace is removed from stored keys, values and section names.
Key values can be omitted, in which case the key-value delimiter (`=`) may also be left out (but this is different from putting a delimiter, we'll
explain it later). You can use comment symbols (`;` and `#` to denote comments). This can be configured with the `set_comment_symbols()` method in the
API. Keep in mind that key-value pairs or section headers cannot span multiple lines.
Owing to how ini files usually are, this means that `[`, `]`, `=`, `;` and `#` are special symbols (this crate will allow you to use `]` sparingly).

Let's take for example:
```INI
[section headers are case-insensitive]
[   section headers are case-insensitive    ]
are the section headers above same? = yes
sectionheaders_and_keysarestored_in_lowercase? = yes
keys_are_also_case_insensitive = Values are case sensitive
;anything after a comment symbol is ignored
#this is also a comment
spaces in keys=allowed ;and everything before this is still valid!
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
The only bit of magic the API does is the section-less properties are put in a section called "default". You can configure this variable via the API.

## Usage
Let's take another simple `ini` file and talk about working with it:
```INI
[topsecret]
KFC = the secret herb is orega-

[values]
Uint = 31415
```
If you read the above sections carefully, you'll know that 1) all the keys are stored in lowercase, 2) `get()` can make access in a case-insensitive
manner and 3) we can use `getuint()` to parse the `Uint` value into an `u64`. Let's see that in action.

```rust
use configparser::ini::Ini;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  let mut config = Ini::new();

  // You can easily load a file to get a clone of the map:
  let map = config.load("tests/test.ini")?;
  println!("{:?}", map);
  // You can also safely not store the reference and access it later with get_map_ref() or get a clone with get_map()

  // If you want to access the value, then you can simply do:
  let val = config.get("TOPSECRET", "KFC").unwrap();
  // Notice how get() can access indexes case-insensitively.

  assert_eq!(val, "the secret herb is orega-"); // value accessible!

  // What if you want remove KFC's secret recipe? Just use set():
  config.set("topsecret", "kfc", None);

  assert_eq!(config.get("TOPSECRET", "KFC"), None); // as expected!

  // What if you want to get an unsigned integer?
  let my_number = config.getuint("values", "Uint")?.unwrap();
  assert_eq!(my_number, 31415); // and we got it!
  // The Ini struct provides more getters for primitive datatypes.

  // You can also access it like a normal hashmap:
  let innermap = map["topsecret"].clone();
  // Remember that all indexes are stored in lowercase!

  // If you want to simply mutate the stored hashmap, you can use get_mut_map()
  let map = config.get_mut_map();
  // You can then use normal HashMap functions on this map at your convenience.

  Ok(())
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
- 0.7.0 (**BETA 3**)
  - Handy getter functions introduced such as `getint()`, `getuint()`, `getfloat()`, `getbool()`
  - Fixed docs
  - Fixed tests
- 0.7.1
  - Enable `Eq` and `PartialEq` traits
  - Improve docs
- 0.8.0 (**BETA 4**)
  - Added feature to set default headers.
  - Added feature to parse from a `String`.
- 0.8.1
  - Added support for comments
  - Improved docs
- 0.9.0 (**BETA 5**)
  - Comment customization is here! (**note:** defaults are now changed to `#` and `;`)
  - Fixed some docs
  - Make more docs pass tests
- 0.9.1
  - Hotfix to change getters to return `Ok(None)` instead of failing parsing for `None` values
- 0.9.2
  - Added `getboolcoerce()` function to parse more `bool`-like values.
  - Convert some snippets to doctests.
- 0.10.0 (**BETA 6**)
  - Added `set()` and `setstr()` methods to add section, key, values to the configuration.
  - Added more test, minor doc fixes.

### Future plans

- Support for `ini::load()` will be dropped in the next major releaser per SemVer (i.e. 1.0.0)
  - It will be replaced with a macro for a similar functionality.
  - It has been marked as deprecated.
- More functions for the `Ini` struct, such as insertion and deletion.
