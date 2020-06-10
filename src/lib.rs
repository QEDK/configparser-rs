/*!A simple configuration parsing utility with no dependencies built on Rust.
`configparser` works on a subset of ini configuration syntax. It is inspired by Python's `configparser`.
This release is experimental, use at your own risk.

# Syntax

You also get a `HashMap` of type `HashMap<String, HashMap<String, String>>` with the `Ini` struct, like:
```rust
use configparser::ini::Ini;

fn main() {
  let config = Ini::new();
  match config.load("Path/to/file...") {
      Err(why) => panic!("{}", why),
      Ok(_) => ()
  };
  // You can then access the map normally like:
  let map = match config.get_map() {
  	None => HashMap::new(), // or whatever you want to if the HashMap is empty
  	Some(map) => map
  }
  for (key, value) in &map {
      println!("{:?}: {:?}", key, value);
  }
  // ...and do what you want with it. :)
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
*/
pub mod ini;