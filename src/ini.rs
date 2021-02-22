//!The ini module provides all the things necessary to load and parse ini-syntax files. The most important of which is the `Ini` struct.
//!See the [implementation](https://docs.rs/configparser/*/configparser/ini/struct.Ini.html) documentation for more details.
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

///The `Ini` struct simply contains a nested hashmap of the loaded configuration, the default section header and comment symbols.
///## Example
///```rust
///use configparser::ini::Ini;
///
///let mut config = Ini::new();
///```
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Ini {
    map: HashMap<String, HashMap<String, Option<String>>>,
    default_section: std::string::String,
    comment_symbols: Vec<char>,
    delimiters: Vec<char>,
    case_sensitive: bool,
}

impl Ini {
    ///Creates a new `HashMap` of `HashMap<String, HashMap<String, Option<String>>>` type for the struct.
    ///All values in the HashMap are stored in `String` type.
    ///## Example
    ///```rust
    ///use configparser::ini::Ini;
    ///
    ///let mut config = Ini::new();
    ///```
    ///Returns the struct and stores it in the calling variable.
    pub fn new() -> Ini {
        Ini {
            map: HashMap::new(),
            default_section: "default".to_owned(),
            comment_symbols: vec![';', '#'],
            delimiters: vec!['=', ':'],
            case_sensitive: false,
        }
    }

    ///Creates a new **case-sensitive** `HashMap` of `HashMap<String, HashMap<String, Option<String>>>` type for the struct.
    ///All values in the HashMap are stored in `String` type.
    ///## Example
    ///```rust
    ///use configparser::ini::Ini;
    ///
    ///let mut config = Ini::new_cs();
    ///```
    ///Returns the struct and stores it in the calling variable.
    pub fn new_cs() -> Ini {
        Ini {
            map: HashMap::new(),
            default_section: "default".to_owned(),
            comment_symbols: vec![';', '#'],
            delimiters: vec!['=', ':'],
            case_sensitive: true,
        }
    }

    ///Sets the default section header to the defined string (the default is `default`).
    ///It must be set before `load()` or `read()` is called in order to take effect.
    ///## Example
    ///```rust
    ///use configparser::ini::Ini;
    ///
    ///let mut config = Ini::new();
    ///
    ///config.set_default_section("topsecret");
    ///let map = config.load("tests/test.ini").unwrap();
    ///```
    ///Returns nothing.
    pub fn set_default_section(&mut self, section: &str) {
        self.default_section = section.to_owned();
    }

    ///Sets the default comment symbols to the defined character slice (the defaults are `;` and `#`).
    ///Keep in mind that this will remove the default symbols. It must be set before `load()` or `read()` is called in order to take effect.
    ///## Example
    ///```rust
    ///use configparser::ini::Ini;
    ///
    ///let mut config = Ini::new();
    ///config.set_comment_symbols(&['!', '#']);
    ///let map = config.load("tests/test.ini").unwrap();
    ///```
    ///Returns nothing.
    pub fn set_comment_symbols(&mut self, symlist: &[char]) {
        self.comment_symbols = symlist.to_vec();
    }

    ///Gets all the sections of the currently-stored `HashMap` in a vector.
    ///## Example
    ///```rust
    ///use configparser::ini::Ini;
    ///
    ///let mut config = Ini::new();
    ///config.load("tests/test.ini");
    ///let sections = config.sections();
    ///```
    ///Returns `Vec<String>`.
    pub fn sections(&self) -> Vec<String> {
        self.map.keys().cloned().collect()
    }

    ///Loads a file from a defined path, parses it and puts the hashmap into our struct.
    ///At one time, it only stores one configuration, so each call to `load()` or `read()` will clear the existing `HashMap`, if present.
    ///## Example
    ///```rust
    ///use configparser::ini::Ini;
    ///
    ///let mut config = Ini::new();
    ///let map = config.load("tests/test.ini").unwrap();  // we can get a clone like this, or just store it
    /////Then, we can use standard hashmap functions like:
    ///let values = map.get("values").unwrap();
    ///```
    ///Returns `Ok(map)` with a clone of the stored `HashMap` if no errors are thrown or else `Err(error_string)`.
    ///Use `get_mut_map()` if you want a mutable reference.
    pub fn load(
        &mut self,
        path: &str,
    ) -> Result<HashMap<String, HashMap<String, Option<String>>>, String> {
        let path = Path::new(path);
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(why) => return Err(format!("couldn't open {}: {}", display, why)),
            Ok(file) => file,
        };

        let mut s = String::new();
        self.map = match file.read_to_string(&mut s) {
            Err(why) => return Err(format!("couldn't read {}: {}", display, why)),
            Ok(_) => match self.parse(s) {
                Err(why) => return Err(why),
                Ok(map) => map,
            },
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
    ///    "[2000s]
    ///    2020 = bad")) {
    ///    Err(why) => panic!("{}", why),
    ///    Ok(inner) => inner
    ///};
    ///let this_year = map["2000s"]["2020"].clone().unwrap();
    ///assert_eq!(this_year, "bad"); // value accessible!
    ///```
    ///Returns `Ok(map)` with a clone of the stored `HashMap` if no errors are thrown or else `Err(error_string)`.
    ///Use `get_mut_map()` if you want a mutable reference.
    pub fn read(
        &mut self,
        input: String,
    ) -> Result<HashMap<String, HashMap<String, Option<String>>>, String> {
        self.map = match self.parse(input) {
            Err(why) => return Err(why),
            Ok(map) => map,
        };
        Ok(self.map.clone())
    }

    ///Writes the current configuation to the specified path. If a file is not present, it is automatically created for you, if a file already
    ///exists, it is truncated and the configuration is written to it.
    ///## Example
    ///```rust
    ///use configparser::ini::Ini;
    ///
    ///fn main() -> std::io::Result<()> {
    ///  let mut config = Ini::new();
    ///  config.read(String::from(
    ///    "[2000s]
    ///    2020 = bad"));
    ///  config.write("output.ini")
    ///}
    ///```
    ///Returns a `std::io::Result<()>` type dependent on whether the write was successful or not.
    pub fn write(&self, path: &str) -> std::io::Result<()> {
        fs::write(path, self.unparse())
    }

    ///Returns a string with the current configuration formatted with valid ini-syntax. This is always safe since the configuration is validated during
    ///parsing.
    ///## Example
    ///```rust
    ///use configparser::ini::Ini;
    ///
    ///let mut config = Ini::new();
    ///config.read(String::from(
    ///  "[2000s]
    ///  2020 = bad"));
    ///let outstring = config.writes();
    ///```
    ///Returns a `String` type contatining the ini-syntax file.
    pub fn writes(&self) -> String {
        self.unparse()
    }

    ///Private function that converts the currently stored configuration into a valid ini-syntax string.
    fn unparse(&self) -> String {
        // push key/value pairs in outmap to out string.
        fn unparse_key_values(out: &mut String, outmap: &HashMap<String, Option<String>>) {
            for (key, val) in outmap.iter() {
                out.push_str(&key);
                if let Some(value) = val {
                    out.push('=');
                    out.push_str(&value);
                }
                out.push('\n');
            }
        }
        let mut out = String::new();
        let mut cloned = self.map.clone();
        if let Some(defaultmap) = cloned.get(&self.default_section) {
            unparse_key_values(&mut out, defaultmap);
            cloned.remove(&self.default_section);
        }
        for (section, secmap) in cloned.iter() {
            out.push_str(&format!("[{}]", section));
            out.push('\n');
            unparse_key_values(&mut out, secmap);
        }
        out
    }

    ///Private function that parses ini-style syntax into a HashMap.
    fn parse(
        &self,
        input: String,
    ) -> Result<HashMap<String, HashMap<String, Option<String>>>, String> {
        let mut map: HashMap<String, HashMap<String, Option<String>>> = HashMap::new();
        let mut section = self.default_section.clone();
        let caser = |val: &str| {
            if self.case_sensitive {
                val.to_owned()
            } else {
                val.to_lowercase()
            }
        };
        for (num, lines) in input.lines().enumerate() {
            let trimmed = match lines.find(|c: char| self.comment_symbols.contains(&c)) {
                Some(idx) => lines[..idx].trim(),
                None => lines.trim(),
            };
            if trimmed.is_empty() {
                continue;
            }
            match trimmed.find('[') {
                Some(start) => match trimmed.rfind(']') {
                    Some(end) => {
                        section = caser(trimmed[start + 1..end].trim());
                    }
                    None => {
                        return Err(format!(
                            "line {}:{}: Found opening bracket for section name but no closing bracket",
                            num, start
                        ));
                    }
                },
                None => match trimmed.find(&self.delimiters[..]) {
                    Some(delimiter) => match map.get_mut(&section) {
                        Some(valmap) => {
                            let key = caser(trimmed[..delimiter].trim());
                            let value = trimmed[delimiter + 1..].trim().to_owned();
                            if key.is_empty() {
                                return Err(format!(
                                    "line {}:{}: Key cannot be empty",
                                    num, delimiter
                                ));
                            } else {
                                valmap.insert(key, Some(value));
                            }
                        }
                        None => {
                            let mut valmap: HashMap<String, Option<String>> = HashMap::new();
                            let key = caser(trimmed[..delimiter].trim());
                            let value = trimmed[delimiter + 1..].trim().to_owned();
                            if key.is_empty() {
                                return Err(format!(
                                    "line {}:{}: Key cannot be empty",
                                    num, delimiter
                                ));
                            } else {
                                valmap.insert(key, Some(value));
                            }
                            map.insert(section.clone(), valmap);
                        }
                    },
                    None => match map.get_mut(&section) {
                        Some(valmap) => {
                            let key = caser(trimmed);
                            valmap.insert(key, None);
                        }
                        None => {
                            let mut valmap: HashMap<String, Option<String>> = HashMap::new();
                            let key = caser(trimmed);
                            valmap.insert(key, None);
                            map.insert(section.clone(), valmap);
                        }
                    },
                },
            }
        }
        Ok(map)
    }

    ///Private function that cases things automatically depending on the set variable.
    fn autocase(&self, section: &str, key: &str) -> (String, String) {
        if self.case_sensitive {
            (section.to_owned(), key.to_owned())
        } else {
            (section.to_lowercase(), key.to_lowercase())
        }
    }

    ///Returns a clone of the stored value from the key stored in the defined section.
    ///Unlike accessing the map directly, `get()` can process your input to make case-insensitive access *if* the
    ///default constructor is used.
    ///All `get` functions will do this automatically under the hood.
    ///## Example
    ///```rust
    ///use configparser::ini::Ini;
    ///
    ///let mut config = Ini::new();
    ///config.load("tests/test.ini");
    ///let value = config.get("default", "defaultvalues").unwrap();
    ///assert_eq!(value, String::from("defaultvalues"));
    ///```
    ///Returns `Some(value)` of type `String` if value is found or else returns `None`.
    pub fn get(&self, section: &str, key: &str) -> Option<String> {
        let (section, key) = self.autocase(section, key);
        self.map.get(&section)?.get(&key)?.clone()
    }

    ///Parses the stored value from the key stored in the defined section to a `bool`.
    ///For ease of use, the function converts the type case-insensitively (`true` == `True`).
    ///## Example
    ///```rust
    ///use configparser::ini::Ini;
    ///
    ///let mut config = Ini::new();
    ///config.load("tests/test.ini");
    ///let value = config.getbool("values", "bool").unwrap().unwrap();
    ///assert!(value);  // value accessible!
    ///```
    ///Returns `Ok(Some(value))` of type `bool` if value is found or else returns `Ok(None)`.
    ///If the parsing fails, it returns an `Err(string)`.
    pub fn getbool(&self, section: &str, key: &str) -> Result<Option<bool>, String> {
        let (section, key) = self.autocase(section, key);
        match self.map.get(&section) {
            Some(secmap) => match secmap.get(&key) {
                Some(val) => match val {
                    Some(inner) => match inner.to_lowercase().parse::<bool>() {
                        Err(why) => Err(why.to_string()),
                        Ok(boolean) => Ok(Some(boolean)),
                    },
                    None => Ok(None),
                },
                None => Ok(None),
            },
            None => Ok(None),
        }
    }

    ///Parses the stored value from the key stored in the defined section to a `bool`. For ease of use, the function converts the type coerces a match.
    ///It attempts to case-insenstively find `true`, `yes`, `t`, `y`, `1` and `on` to parse it as `True`.
    ///Similarly it attempts to case-insensitvely find `false`, `no`, `f`, `n`, `0` and `off` to parse it as `False`.
    ///## Example
    ///```rust
    ///use configparser::ini::Ini;
    ///
    ///let mut config = Ini::new();
    ///config.load("tests/test.ini");
    ///let value = config.getboolcoerce("values", "boolcoerce").unwrap().unwrap();
    ///assert!(!value);  // value accessible!
    ///```
    ///Returns `Ok(Some(value))` of type `bool` if value is found or else returns `Ok(None)`.
    ///If the parsing fails, it returns an `Err(string)`.
    pub fn getboolcoerce(&self, section: &str, key: &str) -> Result<Option<bool>, String> {
        let (section, key) = self.autocase(section, key);
        match self.map.get(&section) {
            Some(secmap) => match secmap.get(&key) {
                Some(val) => match val {
                    Some(inner) => {
                        let boolval = &inner.to_lowercase()[..];
                        if ["true", "yes", "t", "y", "1", "on"].contains(&boolval) {
                            Ok(Some(true))
                        } else if ["false", "no", "f", "n", "0", "off"].contains(&boolval) {
                            Ok(Some(false))
                        } else {
                            Err(format!(
                                "Unable to parse value into bool at {}:{}",
                                section, key
                            ))
                        }
                    }
                    None => Ok(None),
                },
                None => Ok(None),
            },
            None => Ok(None),
        }
    }

    ///Parses the stored value from the key stored in the defined section to an `i64`.
    ///## Example
    ///```rust
    ///use configparser::ini::Ini;
    ///
    ///let mut config = Ini::new();
    ///config.load("tests/test.ini");
    ///let value = config.getint("values", "int").unwrap().unwrap();
    ///assert_eq!(value, -31415);  // value accessible!
    ///```
    ///Returns `Ok(Some(value))` of type `i64` if value is found or else returns `Ok(None)`.
    ///If the parsing fails, it returns an `Err(string)`.
    pub fn getint(&self, section: &str, key: &str) -> Result<Option<i64>, String> {
        let (section, key) = self.autocase(section, key);
        match self.map.get(&section) {
            Some(secmap) => match secmap.get(&key) {
                Some(val) => match val {
                    Some(inner) => match inner.parse::<i64>() {
                        Err(why) => Err(why.to_string()),
                        Ok(int) => Ok(Some(int)),
                    },
                    None => Ok(None),
                },
                None => Ok(None),
            },
            None => Ok(None),
        }
    }

    ///Parses the stored value from the key stored in the defined section to a `u64`.
    ///## Example
    ///```rust
    ///use configparser::ini::Ini;
    ///
    ///let mut config = Ini::new();
    ///config.load("tests/test.ini");
    ///let value = config.getint("values", "Uint").unwrap().unwrap();
    ///assert_eq!(value, 31415);  // value accessible!
    ///```
    ///Returns `Ok(Some(value))` of type `u64` if value is found or else returns `Ok(None)`.
    ///If the parsing fails, it returns an `Err(string)`.
    pub fn getuint(&self, section: &str, key: &str) -> Result<Option<u64>, String> {
        let (section, key) = self.autocase(section, key);
        match self.map.get(&section) {
            Some(secmap) => match secmap.get(&key) {
                Some(val) => match val {
                    Some(inner) => match inner.parse::<u64>() {
                        Err(why) => Err(why.to_string()),
                        Ok(uint) => Ok(Some(uint)),
                    },
                    None => Ok(None),
                },
                None => Ok(None),
            },
            None => Ok(None),
        }
    }

    ///Parses the stored value from the key stored in the defined section to a `f64`.
    ///## Example
    ///```rust
    ///use configparser::ini::Ini;
    ///
    ///let mut config = Ini::new();
    ///config.load("tests/test.ini");
    ///let value = config.getfloat("values", "float").unwrap().unwrap();
    ///assert_eq!(value, 3.1415);  // value accessible!
    ///```
    ///Returns `Ok(Some(value))` of type `f64` if value is found or else returns `Ok(None)`.
    ///If the parsing fails, it returns an `Err(string)`.
    pub fn getfloat(&self, section: &str, key: &str) -> Result<Option<f64>, String> {
        let (section, key) = self.autocase(section, key);
        match self.map.get(&section) {
            Some(secmap) => match secmap.get(&key) {
                Some(val) => match val {
                    Some(inner) => match inner.parse::<f64>() {
                        Err(why) => Err(why.to_string()),
                        Ok(float) => Ok(Some(float)),
                    },
                    None => Ok(None),
                },
                None => Ok(None),
            },
            None => Ok(None),
        }
    }

    ///Returns a clone of the `HashMap` stored in our struct.
    ///## Example
    ///```rust
    ///use configparser::ini::Ini;
    ///
    ///let mut config = Ini::new();
    ///config.read(String::from(
    ///  "[section]
    ///  key=values"));
    ///let map = config.get_map().unwrap();
    ///assert_eq!(map, *config.get_map_ref());  // the cloned map is basically a snapshot that you own
    ///```
    ///Returns `Some(map)` if map is non-empty or else returns `None`.
    ///Similar to `load()` but returns an `Option` type with the currently stored `HashMap`.
    pub fn get_map(&self) -> Option<HashMap<String, HashMap<String, Option<String>>>> {
        if self.map.is_empty() {
            None
        } else {
            Some(self.map.clone())
        }
    }

    ///Returns an immutable reference to the `HashMap` stored in our struct.
    ///## Example
    ///```rust
    ///use configparser::ini::Ini;
    ///
    ///let mut config = Ini::new();
    ///let mapclone = config.read(String::from
    ///  ("[topsecrets]
    ///  Valueless key")).unwrap();
    /////Think of the clone as being a snapshot at a point of time while the reference always points to the current configuration.
    ///assert_eq!(*config.get_map_ref(), mapclone);  // same as expected.
    ///```
    ///If you just need to definitely mutate the map, use `get_mut_map()` instead. Alternatively, you can generate a snapshot by getting a clone
    ///with `get_map()` and work with that.
    pub fn get_map_ref(&self) -> &HashMap<String, HashMap<String, Option<String>>> {
        &self.map
    }

    ///Returns a mutable reference to the `HashMap` stored in our struct.
    ///## Example
    ///```rust
    ///use configparser::ini::Ini;
    ///
    ///let mut config = Ini::new();
    ///config.read(String::from
    ///  ("[topsecrets]
    ///  Valueless key"));
    /////We can then get the mutable map and insert a value like:
    ///config.get_mut_map().get_mut("topsecrets").unwrap().insert(String::from("nuclear launch codes"), None);
    ///assert_eq!(config.get("topsecrets", "nuclear launch codes"), None);  // inserted successfully!
    ///```
    ///If you just need to access the map without mutating, use `get_map_ref()` or make a clone with `get_map()` instead.
    pub fn get_mut_map(&mut self) -> &mut HashMap<String, HashMap<String, Option<String>>> {
        &mut self.map
    }

    ///Sets an `Option<String>` in the `HashMap` stored in our struct. If a particular section or key does not exist, it will be automatically created.
    ///An existing value in the map  will be overwritten. You can also set `None` safely.
    ///## Example
    ///```rust
    ///use configparser::ini::Ini;
    ///
    ///let mut config = Ini::new();
    ///config.read(String::from(
    ///  "[section]
    ///  key=value"));
    ///let key_value = String::from("value");
    ///config.set("section", "key", Some(key_value));
    ///config.set("section", "key", None);  // also valid!
    ///assert_eq!(config.get("section", "key"), None);  // correct!
    ///```
    ///Returns `None` if there is no existing value, else returns `Some(Option<String>)`, with the existing value being the wrapped `Option<String>`.
    ///If you want to insert using a string literal, use `setstr()` instead.
    pub fn set(
        &mut self,
        section: &str,
        key: &str,
        value: Option<String>,
    ) -> Option<Option<String>> {
        let (section, key) = self.autocase(section, key);
        match self.map.get_mut(&section) {
            Some(secmap) => secmap.insert(key, value),
            None => {
                let mut valmap: HashMap<String, Option<String>> = HashMap::new();
                valmap.insert(key, value);
                self.map.insert(section, valmap);
                None
            }
        }
    }

    ///Sets an `Option<&str>` in the `HashMap` stored in our struct. If a particular section or key does not exist, it will be automatically created.
    ///An existing value in the map  will be overwritten. You can also set `None` safely.
    ///## Example
    ///```rust
    ///use configparser::ini::Ini;
    ///
    ///let mut config = Ini::new();
    ///config.read(String::from(
    ///  "[section]
    ///  key=notvalue"));
    ///config.setstr("section", "key", Some("value"));
    ///config.setstr("section", "key", None);  // also valid!
    ///assert_eq!(config.get("section", "key"), None);  // correct!
    ///```
    ///Returns `None` if there is no existing value, else returns `Some(Option<String>)`, with the existing value being the wrapped `Option<String>`.
    ///If you want to insert using a `String`, use `set()` instead.
    pub fn setstr(
        &mut self,
        section: &str,
        key: &str,
        value: Option<&str>,
    ) -> Option<Option<String>> {
        let (section, key) = self.autocase(section, key);
        self.set(&section, &key, value.map(String::from))
    }

    ///Clears the map, removing all sections and properties from the hashmap. It keeps the allocated memory for reuse.
    ///## Example
    ///```rust
    ///use configparser::ini::Ini;
    ///
    ///let mut config = Ini::new();
    ///config.read(String::from(
    ///  "[section]
    ///  key=somevalue"));
    ///config.clear();
    ///assert!(config.get_map_ref().is_empty());  // our map is empty!
    ///```
    ///Returns nothing.
    pub fn clear(&mut self) {
        self.map.clear();
    }

    ///Removes a section from the hashmap, returning the properties stored in the section if the section was previously in the map.
    ///```rust
    ///use configparser::ini::Ini;
    ///
    ///let mut config = Ini::new();
    ///config.read(String::from(
    ///  "[section]
    ///  updog=whatsupdog"));
    ///config.remove_section("section");  // this will return a cloned hashmap of the stored property
    ///assert!(config.get_map_ref().is_empty());  // with the last section removed, our map is now empty!
    ///```
    ///Returns `Some(section_map)` if the section exists or else, `None`.
    pub fn remove_section(&mut self, section: &str) -> Option<HashMap<String, Option<String>>> {
        let section = if self.case_sensitive {
            section.to_owned()
        } else {
            section.to_lowercase()
        };
        self.map.remove(&section)
    }

    ///Removes a key from a section in the hashmap, returning the value attached to the key if it was previously in the map.
    ///```rust
    ///use configparser::ini::Ini;
    ///
    ///let mut config = Ini::new();
    ///config.read(String::from(
    ///  "[section]
    ///  updog=whatsupdog
    ///  [anothersection]
    ///  updog=differentdog"));
    ///let val = config.remove_key("anothersection", "updog").unwrap().unwrap();
    ///assert_eq!(val, String::from("differentdog"));  // with the last section removed, our map is now empty!
    ///```
    ///Returns `Some(Option<String>)` if the value exists or else, `None`.
    pub fn remove_key(&mut self, section: &str, key: &str) -> Option<Option<String>> {
        let (section, key) = self.autocase(section, key);
        self.map.get_mut(&section)?.remove(&key)
    }
}
