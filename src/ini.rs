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
pub fn load(path: &str) -> HashMap<String, HashMap<String, String>> {
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
pub struct Ini {
	map: HashMap<String, HashMap<String, String>>
}

impl Ini {
	///Creates a new `HashMap` for the struct.
	///All values in the HashMap are stored in `String` type.
	///## Example
	///```rust
	///use configparser::ini::Ini;
	///
	///let config = Ini::new();
	///```
	///Returns the struct and stores it in the calling variable.
	pub fn new() -> Ini {
		let map: HashMap<String, HashMap<String, String>> = HashMap::new();
		let inimap = Ini {
			map
		};
		inimap
	}

	///Loads a file from a defined path, parses it and puts the hashmap into our struct.
	///At one time, it only stores one file's configuration, so every call to load() will clear the existing HashMap, if present.
	///## Example
	///```ignore,rust
	///match config.load("Path/to/file...") {
	/// Err(why) => panic!("{}", why),
	/// Ok(_) => ()
	///}
	///```
	///Returns `Ok()` if no errors are thrown or else `Err(error_string)`. Note that you cannot
	///use `?` because the Ok is of `()` type.
	pub fn load(&mut self, path: &str) -> Result<(), String> {
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
		Ok(())
	}

	///Private function that parses ini-style syntax into a HashMap.
	fn parse(&self, input: String) -> Result<HashMap<String, HashMap<String, String>>, String> {
		let mut map: HashMap<String, HashMap<String, String>> = HashMap::new();
		let mut section = "DEFAULT";
		for lines in input.lines() {
			let trimmed = lines.trim();
			match trimmed.find('[') {
				Some(start) => match trimmed.rfind(']') {
					Some(end) => {
						section = &trimmed[start+1..end].trim();
					},
					None => return Err(format!("Found opening bracket at {} but no closing bracket", start))
				}
				None => match trimmed.find('=') {
					Some(delimiter) => {
						match map.get_mut(section) {
							Some(valmap) => {
								valmap.insert(trimmed[..delimiter].trim().to_string(), trimmed[delimiter+1..].trim().to_string());
							}
							None => {
								let valmap: HashMap<String, String> =
								[(trimmed[..delimiter].trim().to_string(), trimmed[delimiter+1..].trim().to_string())]
								.iter().cloned().collect();
								map.insert(section.to_string(), valmap);
							}
						}
					}
					None => ()
				}
			}
		}
		Ok(map)
	}

	///Returns a clone of the stored value from the key stored in the defined section.
	/// ## Example
	///```ignore,rust
	///let value = config.get("section", "key")?;
	///```
	///Returns `Some(value)` of type `String` if value is found or else returns `None`.
	pub fn get(&self, section: &str, key: &str) -> Option<String> {
		match self.map.get(section) {
			Some(innermap) => match innermap.get(key) {
				Some(val) => Some(val.clone()),
				None => None
			},
			None => None
		}
	}

	///Returns a clone of the `HashMap` stored in our struct.
	///## Example
	///```ignore,rust
	///let map = config.get_map()?;
	///```
	///Returns `Some(map)` if map is non-empty or else returns `None`.
	pub fn get_map(&self) -> Option<HashMap<String, HashMap<String, String>>> {
		if self.map.is_empty() { None } else { Some(self.map.clone()) }
	}
}