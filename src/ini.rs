//!The ini module provides all the things necessary to load and parse ini-syntax files. The most important of which is the `Ini` struct.
//!See the [implementation](https://docs.rs/configparser/*/configparser/ini/struct.Ini.html) documentation for more details.
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
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

///The `Ini` struct simply contains a nested hashmap of the loaded configuration, the default section header and comment symbols.
///## Example
///```rust
///use configparser::ini::Ini;
///
///let config = Ini::new();
///```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Ini {
	map: HashMap<String, HashMap<String, Option<String>>>,
	default_section: std::string::String,
	comment_symbols: Vec<char>
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
			map: HashMap::new(),
			default_section: String::from("default"),
			comment_symbols: vec![';', '#']
		}
	}

	///Sets the default section header to the defined string (the default is `default`).
	///It must be set before `load()` or `read()` is called in order to take effect.
	///## Example
	///```ignore,rust
	///let config = Ini::new();
	///config.set_default_section("topsecret");
	///let map = match config.load("Path/to/file...") {
	/// Err(why) => panic!("{}", why),
	/// Ok(inner) => inner
	///};
	///```
	///Returns nothing.
	pub fn set_default_section(&mut self, section: &str) {
		self.default_section = section.to_string();
	}

	///Sets the default comment symbols to something else (the defaults are `;` and `#`). Keep in mind that this will remove the default symbols.
	///It must be set before `load()` or `read()` is called in order to take effect.
	///## Example
	///```ignore,rust
	///let config = Ini::new();
	///config.set_comment_symbols(&['!', '#']);
	///let map = match config.load("Path/to/file...") {
	/// Err(why) => panic!("{}", why),
	/// Ok(inner) => inner
	///};
	///```
	///Returns nothing.
	pub fn set_comment_symbols(&mut self, symlist: &[char]) {
		self.comment_symbols = symlist.to_vec();
	}

	///Loads a file from a defined path, parses it and puts the hashmap into our struct.
	///At one time, it only stores one configuration, so each call to `load()` or `read()` will clear the existing `HashMap`, if present.
	///## Example
	///```ignore,rust
	///let map = match config.load("Path/to/file...") {
	/// Err(why) => panic!("{}", why),
	/// Ok(inner) => inner
	///};
	///let location = map["tupac's"]["crib"].clone().unwrap();
	///```
	///Returns `Ok(map)` with a clone of the stored `HashMap` if no errors are thrown or else `Err(error_string)`.
	///Use `get_mut_map()` if you want a mutable reference.
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

	///Reads an input string, parses it and puts the hashmap into our struct.
	///At one time, it only stores one configuration, so each call to `load()` or `read()` will clear the existing `HashMap`, if present.
	///## Example
	///```rust
	///use configparser::ini::Ini;
	///
	///let mut config = Ini::new();
	///let map = match config.read(String::from(
	///	"[2000s]
	///	2020 = bad"
	///	)) {
	/// Err(why) => panic!("{}", why),
	/// Ok(inner) => inner
	///};
	///let this_year = map["2000s"]["2020"].clone().unwrap();
	///assert_eq!(this_year, "bad"); // value accessible!
	///```
	///Returns `Ok(map)` with a clone of the stored `HashMap` if no errors are thrown or else `Err(error_string)`.
	///Use `get_mut_map()` if you want a mutable reference.
	pub fn read(&mut self, input: String) -> Result<HashMap<String, HashMap<String, Option<String>>>, String> {
		self.map = match self.parse(input) {
			Err(why) => return Err(why),
			Ok(map) => map
		};
		Ok(self.map.clone())
	}

	///Private function that parses ini-style syntax into a HashMap.
	fn parse(&self, input: String) -> Result<HashMap<String, HashMap<String, Option<String>>>, String> {
		let mut map: HashMap<String, HashMap<String, Option<String>>> = HashMap::new();
		let mut section = self.default_section.clone();
		for (num, lines) in input.lines().enumerate() {
			let trimmed = match lines.find(|c: char| self.comment_symbols.contains(&c)) {
				Some(idx) => lines[..idx].trim(),
				None => lines.trim()
			};
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
	///Unlike accessing the map directly, `get()` processes your input to make case-insensitive access.
	///All `get` functions will do this automatically.
	///## Example
	///```ignore,rust
	///let value = config.get("section", "key").unwrap();
	///```
	///Returns `Some(value)` of type `String` if value is found or else returns `None`.
	pub fn get(&self, section: &str, key: &str) -> Option<String> {
		self.map.get(&section.to_lowercase())?.get(&key.to_lowercase())?.clone()
	}

	///Parses the stored value from the key stored in the defined section to a `bool`.
	///For ease of use, the function converts the type case-insensitively (`true` == `True`).
	///## Example
	///```ignore,rust
	///let value = config.getbool("section", "key")?.unwrap();
	///```
	///Returns `Ok(Some(value))` of type `bool` if value is found or else returns `Ok(None)`.
	///If the parsing fails, it returns an `Err(string)`.
	pub fn getbool(&self, section: &str, key: &str) -> Result<Option<bool>, String> {
		match self.map.get(&section.to_lowercase()) {
			Some(secmap) => match secmap.get(&key.to_lowercase()) {
				Some(val) => match val.clone().unwrap().to_lowercase().parse::<bool>() {
					Err(why) => Err(why.to_string()),
					Ok(boolean) => Ok(Some(boolean))
				},
				None => Ok(None)
			},
			None => Ok(None)
		}
	}

	///Parses the stored value from the key stored in the defined section to an `i64`.
	///## Example
	///```ignore,rust
	///let value = config.getint("section", "key")?.unwrap();
	///```
	///Returns `Ok(Some(value))` of type `i64` if value is found or else returns `Ok(None)`.
	///If the parsing fails, it returns an `Err(string)`.
	pub fn getint(&self, section: &str, key: &str) -> Result<Option<i64>, String> {
		match self.map.get(&section.to_lowercase()) {
			Some(secmap) => match secmap.get(&key.to_lowercase()) {
				Some(val) => match val.clone().unwrap().parse::<i64>() {
					Err(why) => Err(why.to_string()),
					Ok(int) => Ok(Some(int))
				},
				None => Ok(None)
			},
			None => Ok(None)
		}
	}

	///Parses the stored value from the key stored in the defined section to a `u64`.
	///## Example
	///```ignore,rust
	///let value = config.getuint("section", "key")?.unwrap();
	///```
	///Returns `Ok(Some(value))` of type `u64` if value is found or else returns `Ok(None)`.
	///If the parsing fails, it returns an `Err(string)`.
	pub fn getuint(&self, section: &str, key: &str) -> Result<Option<u64>, String> {
		match self.map.get(&section.to_lowercase()) {
			Some(secmap) => match secmap.get(&key.to_lowercase()) {
				Some(val) => match val.clone().unwrap().parse::<u64>() {
					Err(why) => Err(why.to_string()),
					Ok(uint) => Ok(Some(uint))
				},
				None => Ok(None)
			},
			None => Ok(None)
		}
	}

	///Parses the stored value from the key stored in the defined section to a `f64`.
	///## Example
	///```ignore,rust
	///let value = config.getfloat("section", "key")?.unwrap();
	///```
	///Returns `Ok(Some(value))` of type `f64` if value is found or else returns `Ok(None)`.
	///If the parsing fails, it returns an `Err(string)`.
	pub fn getfloat(&self, section: &str, key: &str) -> Result<Option<f64>, String> {
		match self.map.get(&section.to_lowercase()) {
			Some(secmap) => match secmap.get(&key.to_lowercase()) {
				Some(val) => match val.clone().unwrap().parse::<f64>() {
					Err(why) => Err(why.to_string()),
					Ok(float) => Ok(Some(float))
				},
				None => Ok(None)
			},
			None => Ok(None)
		}
	}

	///Returns a clone of the `HashMap` stored in our struct.
	///## Example
	///```ignore,rust
	///let map = config.get_map().unwrap();
	///```
	///Returns `Some(map)` if map is non-empty or else returns `None`.
	///Similar to `load()` but returns an `Option` type with the currently stored `HashMap`.
	pub fn get_map(&self) -> Option<HashMap<String, HashMap<String, Option<String>>>> {
		if self.map.is_empty() { None } else { Some(self.map.clone()) }
	}

	///Returns an immutable reference to the `HashMap` stored in our struct.
	///## Example
	///```ignore,rust
	///let map = config.get_map_ref();
	///let sectionmap = map["section name"].clone();
	///```
	///If you just need to definitely mutate the map, use `get_mut_map()` instead.
	pub fn get_map_ref(&self) -> &HashMap<String, HashMap<String, Option<String>>> {
		&self.map
	}

	///Returns a mutable reference to the `HashMap` stored in our struct.
	///##Example
	///```ignore,rust
	///let map = config.get_mut_map();
	///map.get_mut("topsecrets").unwrap().insert(String::from("nuclear launch codes"), None);
	///```
	///If you just need to access the map without mutating, use `get_map_ref()` or make a clone with `get_map()` instead.
	pub fn get_mut_map(&mut self) -> &mut HashMap<String, HashMap<String, Option<String>>> {
		&mut self.map
	}
}