//!The ini module provides all the things necessary to load and parse ini-syntax files.
//!The most important of which is the `Ini` struct.
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;

///A public function of the module to load and parse files into a hashmap.
///Support for this function will be dropped in the near future and replaced with a macro.
#[deprecated(
	since = "0.3.0",
	note = "Please use the Ini struct instead."
	)]
pub fn load(path: &str) -> HashMap<String, HashMap<String, Option<String>>> {
	let mut config = Ini::new();
	match config.load(path) {
		Err(why) => panic!("{}", why),
		Ok(_) => ()
	}
	match config.get_map() {
		Some(map) => map,
		None => HashMap::new()
	}
}

///The `Ini` struct simply contains a nested hashmap of the loaded configuration.
///## Example
///```rust
///use configparser::ini::Ini;
///
///let config = Ini::new();
///```
#[derive(Debug, Clone)]
pub struct Ini {
	map: HashMap<String, HashMap<String, Option<String>>>
}

impl Ini {
	///Creates a new `HashMap` of `HashMap<String, HashMap<String, Option<String>>>` type for the struct.
	///All values in the HashMap are stored in `String` type.
	///## Example
	///```rust
	///use configparser::ini::Ini;
	///
	///let config = Ini::new();
	///```
	///Returns the struct and stores it in the calling variable.
	pub fn new() -> Ini {
		Ini {
			map: HashMap::new()
		}
	}

	///Loads a file from a defined path, parses it and puts the hashmap into our struct.
	///At one time, it only stores one file's configuration, so every call to `load()` will clear the existing HashMap, if present.
	///## Example
	///```ignore,rust
	///let map = match config.load("Path/to/file...") {
	/// Err(why) => panic!("{}", why),
	/// Ok(inner) => inner
	///};
	///```
	///Returns `Ok(map)` with a clone of the stored `HashMap` if no errors are thrown or else `Err(error_string)`.
	///Similar to `get_map()` but returns a `Result` type and requires a path.
	pub fn load(&mut self, path: &str) -> Result<HashMap<String, HashMap<String, Option<String>>>, String> {
		let path = Path::new(path);
		let display = path.display();

		let mut file = match File::open(&path) {
			Err(why) => return Err(format!("couldn't open {}: {}", display, why)),
			Ok(file) => file
		};

		let mut s = String::new();
		self.map = match file.read_to_string(&mut s) {
			Err(why) => return Err(format!("couldn't read {}: {}", display, why)),
			Ok(_) => match self.parse(s) {
				Err(why) => return Err(why),
				Ok(map) => map
			}
		};
		Ok(self.map.clone())
	}

	///Private function that parses ini-style syntax into a HashMap.
	fn parse(&self, input: String) -> Result<HashMap<String, HashMap<String, Option<String>>>, String> {
		let mut map: HashMap<String, HashMap<String, Option<String>>> = HashMap::new();
		let mut section = String::from("DEFAULT");
		for (num, lines) in input.lines().enumerate() {
			let trimmed = lines.trim();
			if trimmed.len() == 0 {
				continue;
			}
			match trimmed.find('[') {
				Some(start) => match trimmed.rfind(']') {
					Some(end) => {
						section = trimmed[start+1..end].trim().to_lowercase();
					},
					None => {
						return Err(format!("line {}:{}: Found opening bracket but no closing bracket", num, start));
					}
				}
				None => match trimmed.find('=') {
					Some(delimiter) => {
						match map.get_mut(&section) {
							Some(valmap) => {
								let key = trimmed[..delimiter].trim().to_lowercase();
								let value = trimmed[delimiter+1..].trim().to_string();
								if key.len() == 0 {
									return Err(format!("line {}:{}: Key cannot be empty", num, delimiter));
								}
								else {
									valmap.insert(key, Some(value));
								}
							},
							None => {
								let mut valmap: HashMap<String, Option<String>> = HashMap::new();
								let key = trimmed[..delimiter].trim().to_lowercase();
								let value = trimmed[delimiter+1..].trim().to_string();
								if key.len() == 0 {
									return Err(format!("line {}:{}: Key cannot be empty", num, delimiter));
								}
								else {
									valmap.insert(key, Some(value));
								}
								map.insert(section.clone(), valmap);
							}
						}
					},
					None => match map.get_mut(&section) {
						Some(valmap) => {
							let key = trimmed.to_lowercase();
							valmap.insert(key, None);
						},
						None => {
							let mut valmap: HashMap<String, Option<String>> = HashMap::new();
							let key = trimmed.to_lowercase();
							valmap.insert(key, None);
							map.insert(section.clone(), valmap);
						}
					}
				}
			}
		}
		Ok(map)
	}

	///Returns a clone of the stored value from the key stored in the defined section.
	/// ## Example
	///```ignore,rust
	///let value = config.get("section", "key").unwrap();
	///```
	///Returns `Some(value)` of type `String` if value is found or else returns `None`.
	pub fn get(&self, section: &str, key: &str) -> Option<String> {
		self.map.get(section)?.get(key)?.clone()
	}

	///Returns a clone of the `HashMap` stored in our struct.
	///## Example
	///```ignore,rust
	///let map = config.get_map().unwrap();
	///```
	///Returns `Some(map)` if map is non-empty or else returns `None`.
	///Similar to load() but returns an `Option` type with the currently stored `HashMap`.
	pub fn get_map(&self) -> Option<HashMap<String, HashMap<String, Option<String>>>> {
		if self.map.is_empty() { None } else { Some(self.map.clone()) }
	}
}