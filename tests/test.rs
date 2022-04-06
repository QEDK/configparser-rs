use configparser::ini::Ini;
use std::error::Error;

#[test]
fn non_cs() -> Result<(), Box<dyn Error>> {
    let mut config = Ini::new();
    let map = config.load("tests/test.ini")?;
    config.set_comment_symbols(&[';', '#', '!']);
    let inpstring = config.read(
        "defaultvalues=defaultvalues
		[topsecret]
		KFC = the secret herb is orega-
                colon:value after colon
		Empty string =
		None string
        Password=[in-brackets]
		[ spacing ]
			indented=indented
		not indented = not indented				;testcomment
		!modified comment
		[values]#another comment
		Bool = True
		Boolcoerce = 0
		Int = -31415
		Uint = 31415
		Float = 3.1415"
            .to_owned(),
    )?;
    assert_eq!(map, inpstring);
    config.set("DEFAULT", "defaultvalues", Some("notdefault".to_owned()));
    assert_eq!(
        config.get("DEFAULT", "defaultvalues").unwrap(),
        vec!["defaultvalues", "notdefault"]
    );
    config.setstr("DEFAULT", "defaultvalues", Some("defaultvalues"));
    assert_eq!(
        config.get("DEFAULT", "defaultvalues").unwrap(),
        vec!["defaultvalues", "notdefault", "defaultvalues"]
    );
    config.setstr("DEFAULT", "defaultvalues", None);
    config.setstr("DEFAULT", "defaultvalues", Some("defaultvalues"));
    config.write("output.ini")?;
    let map2 = config.clone().load("output.ini")?;
    assert_eq!(map2, *config.get_map_ref());
    let map3 = config.clone().read(config.writes())?;
    assert_eq!(map2, map3);
    assert_eq!(config.sections().len(), 4);
    assert_eq!(
        config.get("DEFAULT", "defaultvalues").unwrap(),
        vec!["defaultvalues"]
    );
    assert_eq!(
        config.get("topsecret", "KFC").unwrap(),
        vec!["the secret herb is orega-"]
    );
    assert_eq!(config.get("topsecret", "Empty string").unwrap(), vec![""]);
    assert_eq!(
        config.get("topsecret", "None string").unwrap(),
        Vec::<String>::new()
    );
    assert_eq!(config.get("spacing", "indented").unwrap(), vec!["indented"]);
    assert_eq!(
        config.get("spacing", "not indented").unwrap(),
        vec!["not indented"]
    );
    assert_eq!(
        config.get("topsecret", "colon").unwrap(),
        vec!["value after colon"]
    );
    assert_eq!(config.getbool("values", "Bool").unwrap(), vec![true]);
    assert_eq!(
        config.getboolcoerce("values", "Boolcoerce").unwrap(),
        vec![false]
    );
    assert_eq!(config.getint("values", "Int").unwrap(), vec![-31415]);
    assert_eq!(config.getuint("values", "Uint").unwrap(), vec![31415]);
    assert_eq!(config.getfloat("values", "Float").unwrap(), vec![3.1415]);
    assert_eq!(config.getfloat("topsecret", "None string").unwrap(), vec![]);
    assert_eq!(
        map["default"]["defaultvalues"].clone(),
        vec!["defaultvalues"]
    );
    assert_eq!(
        map["topsecret"]["kfc"].clone(),
        vec!["the secret herb is orega-"]
    );
    assert_eq!(map["topsecret"]["empty string"].clone(), vec![""]);
    assert_eq!(map["topsecret"]["none string"].len(), 0);
    assert_eq!(map["spacing"]["indented"].clone(), vec!["indented"]);
    assert_eq!(map["spacing"]["not indented"].clone(), vec!["not indented"]);
    let mut config2 = config.clone();
    let val = config2.remove_key("default", "defaultvalues");
    assert_eq!(val.unwrap(), vec!["defaultvalues"]);
    assert_eq!(config2.get("default", "defaultvalues"), None);
    config2.remove_section("default");
    assert_eq!(config2.get("default", "nope"), None);
    let mut_map = config.get_mut_map();
    mut_map.get_mut("topsecret").unwrap().insert(
        String::from("none string"),
        vec![String::from("None string")],
    );
    assert_eq!(
        mut_map["topsecret"]["none string"].clone(),
        vec!["None string"]
    );
    mut_map.clear();
    config2.clear();
    assert_eq!(config.get_map_ref(), config2.get_map_ref());
    Ok(())
}

#[test]
fn cs() -> Result<(), Box<dyn Error>> {
    let mut config = Ini::new_cs();
    let map = config.load("tests/test.ini")?;
    config.set_comment_symbols(&[';', '#', '!']);
    let inpstring = config.read(
        "defaultvalues=defaultvalues
        [topsecret]
        KFC = the secret herb is orega-
                colon:value after colon
        Empty string =
        None string
        Password=[in-brackets]
        [ spacing ]
            indented=indented
        not indented = not indented             ;testcomment
        !modified comment
        [values]#another comment
        Bool = True
        Boolcoerce = 0
        Int = -31415
        Uint = 31415
        Float = 3.1415"
            .to_owned(),
    )?;
    assert_eq!(map, inpstring);
    config.set("default", "defaultvalues", Some("notdefault".to_owned()));
    assert_eq!(
        config.get("default", "defaultvalues").unwrap(),
        vec!["defaultvalues", "notdefault"]
    );
    config.setstr("default", "defaultvalues", Some("defaultvalues"));
    assert_eq!(
        config.get("default", "defaultvalues").unwrap(),
        vec!["defaultvalues", "notdefault", "defaultvalues"]
    );
    config.setstr("default", "defaultvalues", None);
    config.setstr("default", "defaultvalues", Some("defaultvalues"));

    config.write("output2.ini")?;
    let map2 = config.clone().load("output2.ini")?;
    assert_eq!(map2, *config.get_map_ref());
    let map3 = config.clone().read(config.writes())?;
    assert_eq!(map2, map3);
    assert_eq!(config.sections().len(), 4);
    assert_eq!(
        config.get("default", "defaultvalues").unwrap(),
        vec!["defaultvalues"]
    );
    assert_eq!(
        config.get("topsecret", "KFC").unwrap(),
        vec!["the secret herb is orega-"]
    );
    assert_eq!(config.get("topsecret", "Empty string").unwrap(), vec![""]);
    assert_eq!(
        config.get("topsecret", "None string").unwrap(),
        Vec::<String>::new()
    );
    assert_eq!(config.get("spacing", "indented").unwrap(), vec!["indented"]);
    assert_eq!(
        config.get("spacing", "not indented").unwrap(),
        vec!["not indented"]
    );
    assert_eq!(
        config.get("topsecret", "colon").unwrap(),
        vec!["value after colon"]
    );
    assert_eq!(config.getbool("values", "Bool").unwrap(), vec![true]);
    assert_eq!(
        config.getboolcoerce("values", "Boolcoerce").unwrap(),
        vec![false]
    );
    assert_eq!(config.getint("values", "Int").unwrap(), vec![-31415]);
    assert_eq!(config.getuint("values", "Uint").unwrap(), vec![31415]);
    assert_eq!(config.getfloat("values", "Float").unwrap(), vec![3.1415]);
    assert_eq!(config.getfloat("topsecret", "None string").unwrap(), vec![]);
    assert_eq!(
        map["default"]["defaultvalues"].clone(),
        vec!["defaultvalues"]
    );
    assert_eq!(
        map["topsecret"]["KFC"].clone(),
        vec!["the secret herb is orega-"]
    );
    assert_eq!(map["topsecret"]["Empty string"].clone(), vec![""]);
    assert_eq!(map["topsecret"]["None string"].len(), 0);
    assert_eq!(map["spacing"]["indented"].clone(), vec!["indented"]);
    assert_eq!(map["spacing"]["not indented"].clone(), vec!["not indented"]);
    let mut config2 = config.clone();
    let val = config2.remove_key("default", "defaultvalues");
    assert_eq!(val.unwrap(), ["defaultvalues"]);
    assert_eq!(config2.get("default", "defaultvalues"), None);
    config2.remove_section("default");
    assert_eq!(config2.get("default", "nope"), None);
    let mut_map = config.get_mut_map();
    mut_map.get_mut("topsecret").unwrap().insert(
        String::from("none string"),
        vec![String::from("None string")],
    );
    assert_eq!(
        mut_map["topsecret"]["none string"].clone(),
        vec!["None string"]
    );
    mut_map.clear();
    config2.clear();
    assert_eq!(config.get_map_ref(), config2.get_map_ref());
    Ok(())
}

#[test]
#[cfg(feature = "indexmap")]
fn sort_on_write() -> Result<(), Box<dyn Error>> {
    let mut config = Ini::new_cs();
    config.load("tests/test.ini")?;

    assert_eq!(
        config.writes(),
        "defaultvalues=defaultvalues
[topsecret]
KFC=the secret herb is orega-
colon=value after colon
Empty string=
None string
Password=[in-brackets]
[spacing]
indented=indented
not indented=not indented
[values]
Bool=True
Boolcoerce=0
Int=-31415
Uint=31415
Float=3.1415
"
    );

    Ok(())
}
