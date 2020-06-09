//!ini
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;

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