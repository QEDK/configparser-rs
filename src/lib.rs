/*!A simple configuration parsing utility with no dependencies built on Rust.
`configparser` works on a subset of ini configuration syntax. It is inspired by Python's `configparser`.
The current release is experimental, this means that future releases will be swift until we reach `stable` (1.0.0).
The codebase is thus subject to change for now.

# ini-style configuration

Most `ini` files look something like this (but they don't need to be `.ini` files obviously):
```yaml
[some-section]
key1 = value1
key2 = value2

[some-other-section]
key3 = value3
maybekey1aswell = value1
```
Owing to how ini files usually are, this means that `[, ], =` are special symbols (this crate will allow you to use `]` sparingly).
Key-value pairs or section headers cannot spread across multiple lines for obvious reasons as well because the parser cannot reliably parse it otherwise.
A value on the next line could as well be a key for another.
An important note is that key-value pairs not attached to any section are automatically put in a section called 'DEFAULT'.
Future releases will add support for escaping, comments and modifying default sections.

# Syntax

You can get a `HashMap` of type `HashMap<String, HashMap<String, String>>` with the `Ini` struct, like:
```ignore
use configparser::ini::Ini;
use std::collections::HashMap;

fn main() {
  let mut config = Ini::new();
  match config.load("Path/to/file...") {
      Err(why) => panic!("{}", why),
      Ok(_) => ()
  };
  // You can then access the map normally like:
  let map = match config.get_map() {
  	None => HashMap::new(), // or whatever you want to if the HashMap is empty
  	Some(map) => map
  };
  for (key, value) in &map {
      println!("{:?}: {:?}", key, value);
  }
  // ...and do what you want with it. :)
}
```
The `Ini` struct is the way to go forward and will soon have more features, such as reading from a string, insertion, deletion and variable access.

As of now, there's also a public function, to load an ini-syntax file and parse it into a hashmap. Support for this will be dropped in the near future, and will be changed into a macro when it's dropped.
```ignore
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
*/
pub mod ini;