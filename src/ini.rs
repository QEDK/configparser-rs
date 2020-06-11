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
	let path = Path::new(path);
	let display = path.display();

	let mut map: HashMap<String, HashMap<String, String>> = HashMap::new();

	let mut file = match File::open(&path) {
		Err(why) => panic!("couldn't open {}: {}", display, why),
		Ok(file) => file,
	};

	let mut s = String::new();
	let mut section = "DEFAULT";
	match file.read_to_string(&mut s) {
		Err(why) => panic!("couldn't read {}: {}", display, why),
		Ok(_) => for lines in s.lines() {
			let trimmed = lines.trim();
			match trimmed.find('[') {
				Some(start) => match trimmed.rfind(']') {
					Some(end) => {
						section = &trimmed[start+1..end];
					},
					None => panic!("Found opening bracket at {} but no closing bracket", start)
				}
				None => match trimmed.find('=') {
					Some(delimiter) => {
						match map.get_mut(section) {
							Some(valmap) => {
								valmap.insert(trimmed[..delimiter].to_string(), trimmed[delimiter+1..].to_string());
							}
							None => {
								let valmap: HashMap<String, String> =
								[(trimmed[..delimiter].to_string(), trimmed[delimiter+1..].to_string())]
								.iter().cloned().collect();
								map.insert(section.to_string(), valmap);
							}
						}
					}
					None => ()
				}
			}
		}
	}
	return map;
}

///The `Ini` struct simply contains a hashmap of the loaded configuration.
pub struct Ini {
	map: HashMap<String, HashMap<String, String>>
}

impl Ini {
	///Creates a new `HashMap` for the struct.
	///Example: `let config = Ini::new();`
	///Returns the struct and stores it in the calling variable.
	pub fn new() -> Ini {
		let map: HashMap<String, HashMap<String, String>> = HashMap::new();
		let inimap = Ini {
			map
		};
		inimap
	}

	///Loads a file from a defined path, parses it and adds the hashmap into our struct.
	///Example: `config.load("Path/to/file...")`
	///Returns Ok() if no erros are thrown or else Err(error_string)
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
						section = &trimmed[start+1..end];
					},
					None => return Err(format!("Found opening bracket at {} but no closing bracket", start))
				}
				None => match trimmed.find('=') {
					Some(delimiter) => {
						match map.get_mut(section) {
							Some(valmap) => {
								valmap.insert(trimmed[..delimiter].to_string(), trimmed[delimiter+1..].to_string());
							}
							None => {
								let valmap: HashMap<String, String> =
								[(trimmed[..delimiter].to_string(), trimmed[delimiter+1..].to_string())]
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

	///Returns the stored value from the key stored in the defined section.
	///Example: `let value: String = config.get("section", "key")`
	///Returns Some(value) if value is found or else returns None.
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
	///Example: `let map = config.get_map()`
	///Returns Some(map) if map is non-empty or else returns None.
	pub fn get_map(&self) -> Option<HashMap<String, HashMap<String, String>>> {
		if self.map.is_empty() { None } else { Some(self.map.clone()) }
	}
}