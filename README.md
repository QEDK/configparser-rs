# configparser [![Build Status](https://travis-ci.com/QEDK/configparser-rs.svg?branch=master)](https://travis-ci.com/QEDK/configparser-rs)
A simple configuration parsing utility with no dependencies built on Rust.

`configparser` works on a subset of ini configuration syntax.

Inspired by Python's `configparser`. This release is experimental, use at your own risk.

## Installation
You can install this easily via `cargo` by including it in your `Cargo.toml` file like:
```yaml
[dependencies]
configparser = "0.1.1"
```

## Usage
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

### Future plans

- Support for `ini::load()` will be dropped in the next major releaser per SemVer (i.e. 1.0.0)
 - It will be replaced with a macro for a similar functionality.
 - It will be marked as deprecated in the next release.
- More functions for `Ini` struct, such as reading from a string, insertion, deletion.