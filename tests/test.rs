use configparser::ini::Ini;
use std::error::Error;

#[test]
fn main() -> Result<(), Box<dyn Error>> {
	let mut config = Ini::new();
	let map = config.load("tests/test.ini")?;
	let inpstring = config.read(String::from
		("defaultvalues=defaultvalues
		[topsecret]
		KFC = the secret herb is orega-
		Empty string =
		None string
		[ spacing ]
			indented=indented
		not indented = not indented

		[values]
		Bool = True
		Int = -31415
		Uint = 31415
		Float = 3.1415"))?;
	assert_eq!(map, inpstring);
	assert_eq!(config.get("DEFAULT", "defaultvalues").unwrap(), "defaultvalues");
	assert_eq!(config.get("topsecret", "KFC").unwrap(), "the secret herb is orega-");
	assert_eq!(config.get("topsecret", "Empty string").unwrap(), "");
	assert_eq!(config.get("topsecret", "None string"), None);
	assert_eq!(config.get("spacing", "indented").unwrap(), "indented");
	assert_eq!(config.get("spacing", "not indented").unwrap(), "not indented");
	assert_eq!(config.getbool("values", "Bool")?.unwrap(), true);
	assert_eq!(config.getint("values", "Int")?.unwrap(), -31415);
	assert_eq!(config.getuint("values", "Uint")?.unwrap(), 31415);
	assert_eq!(config.getfloat("values", "Float")?.unwrap(), 3.1415);
	assert_eq!(map["default"]["defaultvalues"].clone().unwrap(), "defaultvalues");
	assert_eq!(map["topsecret"]["kfc"].clone().unwrap(), "the secret herb is orega-");
	assert_eq!(map["topsecret"]["empty string"].clone().unwrap(), "");
	assert_eq!(map["topsecret"]["none string"], None);
	assert_eq!(map["spacing"]["indented"].clone().unwrap(), "indented");
	assert_eq!(map["spacing"]["not indented"].clone().unwrap(), "not indented");
	let mut_map = config.get_mut_map();
	mut_map.get_mut("topsecret").unwrap().insert(String::from("none string"), Some(String::from("None string")));
	assert_eq!(mut_map["topsecret"]["none string"].clone().unwrap(), "None string");
	mut_map.clear();
	assert!(config.get_map_ref().is_empty());
	Ok(())
}

#[test]#[allow(unused_variables)]
fn doc() -> Result<(), Box<dyn Error>> {
  let mut config = Ini::new();

  // You can easily load a file to get a clone of the map:
  let map = config.load("tests/test.ini")?;
  println!("{:?}", map);
  // You can also safely not store the reference and access it later with get_map_ref() or get a clone with get_map()

  // You can then access it like a normal hashmap:
  let innermap = map["topsecret"].clone(); // Remember this is a hashmap!

  // If you want to access the value, then you can simply do:
  let val = map["topsecret"]["kfc"].clone().unwrap();
  // Lowercasing when accessing map directly is important because all keys are stored in lower-case!
  // Note: The .clone().unwrap() is required because it's an Option<String> type.

  assert_eq!(val, "the secret herb is orega-"); // value accessible!

  // What if you want to mutate the parser and remove KFC's secret recipe? Just use get_mut_map():
  let mut_map = config.get_mut_map();
  mut_map.get_mut("topsecret").unwrap().insert(String::from("kfc"), None);
  // And the secret is back in safety, remember that these are normal HashMap functions chained for convenience.

  // However very quickly see how that becomes cumbersome, so you can use the handy get() function from Ini
  // The get() function accesses the map case-insensitively, so you can use uppercase as well:
  let val = config.get("topsecret", "KFC"); // unwrapping will be an error because we just emptied it!
  assert_eq!(val, None); // as expected!

  // What if you want to get a number?
  let my_number = config.getint("values", "Int")?.unwrap();
  assert_eq!(my_number, -31415); // and we got it!
  // The Ini struct provides more getters for primitive datatypes.

  Ok(())
}