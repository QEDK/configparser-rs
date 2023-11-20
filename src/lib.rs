/*!
This crate provides the `Ini` struct which implements a basic configuration language which provides a structure similar to what’s found in Windows' `ini` files.
You can use this to write Rust programs which can be customized by end users easily.

This is a simple configuration parsing utility with no dependencies built on Rust. It is inspired by Python's `configparser`.

The current release is stable and changes will take place at a slower pace. We'll be keeping semver in mind for future releases as well.

## 🚀 Quick Start

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
Essentially, the syntax consists of sections, each of which can which contains keys with values. The `Ini` struct can read and write such values to
strings as well as files.

## ➕ Supported datatypes
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


## 📝 Supported `ini` file structure
A configuration file can consist of sections, each led by a `[section-name]` header, followed by key-value entries separated by a delimiter (`=` and `:`). By default, section names and key names are case-insensitive. Case-sensitivity can be enabled using the `Ini::new_cs()` constructor. All leading and trailing whitespace is removed from stored keys, values and section names.
Key values can be omitted, in which case the key-value delimiter
may also be left out (but this is different from putting a delimiter, we'll
explain it later). You can use comment symbols (`;` and `#` to denote comments). This can be configured with the `set_comment_symbols()` method in the
API. Keep in mind that key-value pairs or section headers cannot span multiple lines.
Owing to how ini files usually are, this means that `[`, `]`, `=`, `:`, `;` and `#` are special symbols by default (this crate will allow you to use `]` sparingly).
Let's take for example:
```INI
[section headers are case-insensitive by default]
[   section headers are case-insensitive by default   ]
are the section headers above same? = yes
sectionheaders_and_keysarestored_in_lowercase? = yes
keys_are_also_case_insensitive = Values are case sensitive
Case-sensitive_keys_and_sections = using a special constructor
you can also use colons : instead of the equal symbol
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
Keep in mind that a section named "default" is also treated as sectionless so the output files remains consistent with no section header.

## Usage
Let's take another simple `ini` file and talk about working with it:
```INI
[topsecret]
KFC = the secret herb is orega-

[values]
Uint = 31415
```
If you read the above sections carefully, you'll know that 1) all the keys are stored in lowercase, 2) `get()` can make access in a case-insensitive
manner and 3) we can use `getint()` to parse the `Int` value into an `i64`. Let's see that in action.

```rust
use configparser::ini::{Ini, WriteOptions};
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

  // You can easily write the currently stored configuration to a file with the `write` method. This creates a compact format with as little spacing as possible:
  config.write("output.ini");

  // You can write the currently stored configuration with different spacing to a file with the `pretty_write` method:
  let write_options = WriteOptions::new_with_params(true, 2, 1);
  // or you can use the default configuration as `WriteOptions::new()`
  config.pretty_write("pretty_output.ini", &write_options);

  // If you want to simply mutate the stored hashmap, you can use get_mut_map()
  let map = config.get_mut_map();
  // You can then use normal HashMap functions on this map at your convenience.
  // Remember that functions which rely on standard formatting might stop working
  // if it's mutated differently.

  // If you want a case-sensitive map, just do:
  let mut config = Ini::new_cs();
  // This automatically changes the behaviour of every function and parses the file as case-sensitive.

  Ok(())
}
```
*/
pub mod ini;
