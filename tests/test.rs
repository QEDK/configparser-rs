use configparser::ini::Ini;
use std::collections::HashSet;
use std::error::Error;

#[cfg(feature = "indexmap")]
use configparser::ini::WriteOptions;

#[test]
#[allow(clippy::approx_constant)]
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
        "notdefault"
    );
    config.setstr("DEFAULT", "defaultvalues", Some("defaultvalues"));
    assert_eq!(
        config.get("DEFAULT", "defaultvalues").unwrap(),
        "defaultvalues"
    );
    config.setstr("DEFAULT", "defaultvalues", None);
    config.write("output.ini")?;
    let map2 = config.clone().load("output.ini")?;
    assert_eq!(map2, *config.get_map_ref());
    let map3 = config.clone().read(config.writes())?;
    assert_eq!(map2, map3);
    assert_eq!(config.sections().len(), 4);
    assert_eq!(config.get("DEFAULT", "defaultvalues"), None);
    assert_eq!(
        config.get("topsecret", "KFC").unwrap(),
        "the secret herb is orega-"
    );
    assert_eq!(config.get("topsecret", "Empty string").unwrap(), "");
    assert_eq!(config.get("topsecret", "None string"), None);
    assert_eq!(config.get("spacing", "indented").unwrap(), "indented");
    assert_eq!(
        config.get("spacing", "not indented").unwrap(),
        "not indented"
    );
    assert_eq!(
        config.get("topsecret", "colon").unwrap(),
        "value after colon"
    );
    assert!(config.getbool("values", "Bool")?.unwrap());
    assert!(!config.getboolcoerce("values", "Boolcoerce")?.unwrap());
    assert_eq!(config.getint("values", "Int")?.unwrap(), -31415);
    assert_eq!(config.getuint("values", "Uint")?.unwrap(), 31415);
    assert_eq!(config.getfloat("values", "Float")?.unwrap(), 3.1415);
    assert_eq!(config.getfloat("topsecret", "None string"), Ok(None));
    assert_eq!(
        map["default"]["defaultvalues"].clone().unwrap(),
        "defaultvalues"
    );
    assert_eq!(
        map["topsecret"]["kfc"].clone().unwrap(),
        "the secret herb is orega-"
    );
    assert_eq!(map["topsecret"]["empty string"].clone().unwrap(), "");
    assert_eq!(map["topsecret"]["none string"], None);
    assert_eq!(map["spacing"]["indented"].clone().unwrap(), "indented");
    assert_eq!(
        map["spacing"]["not indented"].clone().unwrap(),
        "not indented"
    );
    let mut config2 = config.clone();
    let val = config2.remove_key("default", "defaultvalues");
    assert_eq!(val, Some(None));
    assert_eq!(config2.get("default", "defaultvalues"), None);
    config2.remove_section("default");
    assert_eq!(config2.get("default", "nope"), None);
    let mut_map = config.get_mut_map();
    mut_map.get_mut("topsecret").unwrap().insert(
        String::from("none string"),
        Some(String::from("None string")),
    );
    assert_eq!(
        mut_map["topsecret"]["none string"].clone().unwrap(),
        "None string"
    );
    mut_map.clear();
    config2.clear();
    assert_eq!(config.get_map_ref(), config2.get_map_ref());

    config.load("tests/test.ini")?;
    config.read_and_append("defaultvalues=somenewvalue".to_owned())?;
    assert_eq!(
        config.get("default", "defaultvalues").unwrap(),
        "somenewvalue"
    );
    assert_eq!(
        config.get("topsecret", "KFC").unwrap(),
        "the secret herb is orega-"
    );

    let mut config3 = config.clone();
    let mut_map = config3.get_mut_map();
    mut_map.clear();
    config3.load("tests/test.ini")?;
    config3.load_and_append("tests/test_more.ini")?;
    assert_eq!(
        config3.get("default", "defaultvalues").unwrap(),
        "overwritten"
    );
    assert_eq!(config3.get("topsecret", "KFC").unwrap(), "redacted");
    // spacing -> indented exists in tests/test.ini, but not tests/test_more.ini
    assert_eq!(config3.get("spacing", "indented").unwrap(), "indented");
    assert!(!config3.getbool("values", "Bool")?.unwrap());

    Ok(())
}

#[test]
#[allow(clippy::approx_constant)]
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
        "notdefault"
    );
    config.setstr("default", "defaultvalues", Some("defaultvalues"));
    assert_eq!(
        config.get("default", "defaultvalues").unwrap(),
        "defaultvalues"
    );
    config.setstr("default", "defaultvalues", None);
    config.write("output2.ini")?;
    let map2 = config.clone().load("output2.ini")?;
    assert_eq!(map2, *config.get_map_ref());
    let map3 = config.clone().read(config.writes())?;
    assert_eq!(map2, map3);
    assert_eq!(config.sections().len(), 4);
    assert_eq!(config.get("default", "defaultvalues"), None);
    assert_eq!(
        config.get("topsecret", "KFC").unwrap(),
        "the secret herb is orega-"
    );
    assert_eq!(config.get("topsecret", "Empty string").unwrap(), "");
    assert_eq!(config.get("topsecret", "None string"), None);
    assert_eq!(config.get("spacing", "indented").unwrap(), "indented");
    assert_eq!(
        config.get("spacing", "not indented").unwrap(),
        "not indented"
    );
    assert_eq!(
        config.get("topsecret", "colon").unwrap(),
        "value after colon"
    );
    assert!(config.getbool("values", "Bool")?.unwrap());
    assert!(!config.getboolcoerce("values", "Boolcoerce")?.unwrap());
    assert_eq!(config.getint("values", "Int")?.unwrap(), -31415);
    assert_eq!(config.getuint("values", "Uint")?.unwrap(), 31415);
    assert_eq!(config.getfloat("values", "Float")?.unwrap(), 3.1415);
    assert_eq!(config.getfloat("topsecret", "None string"), Ok(None));
    assert_eq!(
        map["default"]["defaultvalues"].clone().unwrap(),
        "defaultvalues"
    );
    assert_eq!(
        map["topsecret"]["KFC"].clone().unwrap(),
        "the secret herb is orega-"
    );
    assert_eq!(map["topsecret"]["Empty string"].clone().unwrap(), "");
    assert_eq!(map["topsecret"]["None string"], None);
    assert_eq!(map["spacing"]["indented"].clone().unwrap(), "indented");
    assert_eq!(
        map["spacing"]["not indented"].clone().unwrap(),
        "not indented"
    );
    let mut config2 = config.clone();
    let val = config2.remove_key("default", "defaultvalues");
    assert_eq!(val, Some(None));
    assert_eq!(config2.get("default", "defaultvalues"), None);
    config2.remove_section("default");
    assert_eq!(config2.get("default", "nope"), None);
    let mut_map = config.get_mut_map();
    mut_map.get_mut("topsecret").unwrap().insert(
        String::from("none string"),
        Some(String::from("None string")),
    );
    assert_eq!(
        mut_map["topsecret"]["none string"].clone().unwrap(),
        "None string"
    );
    mut_map.clear();
    config2.clear();
    assert_eq!(config.get_map_ref(), config2.get_map_ref());
    Ok(())
}

#[test]
fn ensure_empty_sections_exist() -> Result<(), Box<dyn Error>> {
    const FILE_CONTENTS: &str = "
[basic_section]
basic_option=basic_value
[empty_section]
";

    let mut config = Ini::new();
    config.read(FILE_CONTENTS.to_owned())?;

    assert_eq!(
        HashSet::from_iter(config.sections().into_iter()),
        HashSet::from([String::from("basic_section"), String::from("empty_section")])
    );

    Ok(())
}

#[test]
fn inline_comment_symbols() -> Result<(), Box<dyn Error>> {
    const FILE_CONTENTS: &str = "
[basic_section]
; basic comment
 ; comment with space
! extra_comment=
basic_option=value
basic_with_comment=value ; Simple comment
basic_with_extra_inline=value ! comment
empty_option=
";

    let mut config = Ini::new();
    config.read(FILE_CONTENTS.to_owned())?;

    assert_eq!(
        config.get("basic_section", "basic_option"),
        Some(String::from("value"))
    );
    assert_eq!(
        config.get("basic_section", "basic_with_comment"),
        Some(String::from("value"))
    );
    assert_eq!(
        config.get("basic_section", "basic_with_extra_inline"),
        Some(String::from("value ! comment"))
    );
    assert_eq!(
        config.get("basic_section", "! extra_comment"),
        Some(String::from(""))
    );

    assert_eq!(
        config.get("basic_section", "empty_option"),
        Some(String::from(""))
    );

    config.set_inline_comment_symbols(Some(&['!']));

    config.read(FILE_CONTENTS.to_owned())?;

    assert_eq!(
        config.get("basic_section", "basic_option"),
        Some(String::from("value"))
    );
    assert_eq!(
        config.get("basic_section", "basic_with_comment"),
        Some(String::from("value ; Simple comment"))
    );
    assert_eq!(
        config.get("basic_section", "basic_with_extra_inline"),
        Some(String::from("value"))
    );

    config.set_inline_comment_symbols(Some(&[]));

    config.read(FILE_CONTENTS.to_owned())?;

    assert_eq!(
        config.get("basic_section", "basic_option"),
        Some(String::from("value"))
    );
    assert_eq!(
        config.get("basic_section", "basic_with_comment"),
        Some(String::from("value ; Simple comment"))
    );
    assert_eq!(
        config.get("basic_section", "basic_with_extra_inline"),
        Some(String::from("value ! comment"))
    );

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

#[test]
#[cfg(feature = "indexmap")]
fn pretty_writes_result_is_formatted_correctly() -> Result<(), Box<dyn Error>> {
    use configparser::ini::IniDefault;

    const OUT_FILE_CONTENTS: &str = "defaultvalues=defaultvalues
[topsecret]
KFC=the secret herb is orega-
Empty string=
None string
Password=[in-brackets]
[Section]
Key1: Value1
Key2: this is a haiku
    spread across separate lines
    a single value
Key3: another value
";

    let mut ini_defaults = IniDefault::default();
    ini_defaults.case_sensitive = true;
    ini_defaults.multiline = true;
    let mut config = Ini::new_from_defaults(ini_defaults);
    config.read(OUT_FILE_CONTENTS.to_owned())?;

    let mut write_options = WriteOptions::default();
    write_options.space_around_delimiters = true;
    write_options.multiline_line_indentation = 2;
    write_options.blank_lines_between_sections = 1;
    assert_eq!(
        config.pretty_writes(&write_options),
        "defaultvalues = defaultvalues

[topsecret]
KFC = the secret herb is orega-
Empty string =
None string
Password = [in-brackets]

[Section]
Key1 = Value1
Key2 = this is a haiku
  spread across separate lines
  a single value
Key3 = another value
"
    );

    Ok(())
}

#[test]
#[cfg(feature = "indexmap")]
#[cfg(feature = "async-std")]
#[cfg(feature = "tokio")]
fn pretty_write_result_is_formatted_correctly() -> Result<(), Box<dyn Error>> {
    use configparser::ini::IniDefault;

    const OUT_FILE_CONTENTS: &str = "defaultvalues=defaultvalues
[topsecret]
KFC=the secret herb is orega-
Empty string=
None string
Password=[in-brackets]
[Section]
Key1: Value1
Key2: this is a haiku
    spread across separate lines
    a single value
Key3: another value
";

    let mut ini_defaults = IniDefault::default();
    ini_defaults.case_sensitive = true;
    ini_defaults.multiline = true;
    let mut config = Ini::new_from_defaults(ini_defaults);
    config.read(OUT_FILE_CONTENTS.to_owned())?;

    let mut write_options = WriteOptions::default();
    write_options.space_around_delimiters = true;
    write_options.multiline_line_indentation = 2;
    write_options.blank_lines_between_sections = 1;
    config.pretty_write("pretty_output.ini", &write_options)?;

    let file_contents = std::fs::read_to_string("pretty_output.ini")?;
    assert_eq!(
        file_contents,
        "defaultvalues = defaultvalues

[topsecret]
KFC = the secret herb is orega-
Empty string =
None string
Password = [in-brackets]

[Section]
Key1 = Value1
Key2 = this is a haiku
  spread across separate lines
  a single value
Key3 = another value
"
    );

    Ok(())
}

#[tokio::test]
#[cfg(feature = "indexmap")]
#[cfg(feature = "async-std")]
#[cfg(feature = "tokio")]
async fn async_pretty_print_result_is_formatted_correctly() -> Result<(), Box<dyn Error>> {
    use configparser::ini::IniDefault;

    const OUT_FILE_CONTENTS: &str = "defaultvalues=defaultvalues
[topsecret]
KFC=the secret herb is orega-
Empty string=
None string
Password=[in-brackets]
[Section]
Key1: Value1
Key2: this is a haiku
    spread across separate lines
    a single value
Key3: another value
";

    let mut ini_defaults = IniDefault::default();
    ini_defaults.case_sensitive = true;
    ini_defaults.multiline = true;
    let mut config = Ini::new_from_defaults(ini_defaults);
    config.read(OUT_FILE_CONTENTS.to_owned())?;

    let mut write_options = WriteOptions::default();
    write_options.space_around_delimiters = true;
    write_options.multiline_line_indentation = 2;
    write_options.blank_lines_between_sections = 1;
    config
        .pretty_write_async("pretty_output_async.ini", &write_options)
        .await
        .map_err(|e| e.to_string())?;

    let file_contents = std::fs::read_to_string("pretty_output_async.ini")?;
    assert_eq!(
        file_contents,
        "defaultvalues = defaultvalues

[topsecret]
KFC = the secret herb is orega-
Empty string =
None string
Password = [in-brackets]

[Section]
Key1 = Value1
Key2 = this is a haiku
  spread across separate lines
  a single value
Key3 = another value
"
    );

    Ok(())
}

#[tokio::test]
#[cfg(feature = "async-std")]
#[cfg(feature = "tokio")]
async fn async_load_write() -> Result<(), Box<dyn Error>> {
    const OUT_FILE_CONTENTS: &str = "defaultvalues=defaultvalues
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
    Float = 3.1415";

    let mut config = Ini::new();
    config.read(OUT_FILE_CONTENTS.to_owned())?;
    config.write("output_sync.ini")?;

    let mut config_async = Ini::new();
    config_async.read(OUT_FILE_CONTENTS.to_owned())?;
    config_async
        .write_async("output_async.ini")
        .await
        .map_err(|e| e.to_string())?;

    let mut sync_content = Ini::new();
    sync_content.load("output_sync.ini")?;

    let mut async_content = Ini::new();
    async_content.load_async("output_async.ini").await?;

    assert_eq!(sync_content, async_content);

    Ok(())
}

#[tokio::test]
#[cfg(feature = "async-std")]
#[cfg(feature = "tokio")]
async fn async_load_and_append() -> Result<(), Box<dyn Error>> {
    let mut sync_content = Ini::new();
    sync_content.load("tests/test.ini")?;
    sync_content.load_and_append("tests/test_more.ini")?;

    let mut async_content = Ini::new();
    async_content.load_async("tests/test.ini").await?;
    async_content
        .load_and_append_async("tests/test_more.ini")
        .await?;

    assert_eq!(sync_content, async_content);

    Ok(())
}

#[test]
#[cfg(feature = "indexmap")]
fn multiline_off() -> Result<(), Box<dyn Error>> {
    let mut config = Ini::new_cs();
    config.load("tests/test_multiline.ini")?;

    let map = config.get_map_ref();

    let section = map.get("Section").unwrap();

    assert_eq!(config.get("Section", "Key1").unwrap(), "Value1");
    assert_eq!(config.get("Section", "Key2").unwrap(), "Value Two");
    assert_eq!(config.get("Section", "Key3").unwrap(), "this is a haiku");
    assert!(section.contains_key("spread across separate lines"));
    assert!(section.contains_key("a single value"));

    assert_eq!(config.get("Section", "Key4").unwrap(), "Four");

    assert_eq!(
        config.writes(),
        "[Section]
Key1=Value1
Key2=Value Two
Key3=this is a haiku
spread across separate lines
a single value
Key4=Four
"
    );

    Ok(())
}

#[test]
#[cfg(feature = "indexmap")]
fn multiline_on() -> Result<(), Box<dyn Error>> {
    let mut config = Ini::new_cs();
    config.set_multiline(true);
    config.load("tests/test_multiline.ini")?;

    assert_eq!(config.get("Section", "Key1").unwrap(), "Value1");
    assert_eq!(config.get("Section", "Key2").unwrap(), "Value Two");
    assert_eq!(
        config.get("Section", "Key3").unwrap(),
        "this is a haiku\nspread across separate lines\n\na single value"
    );
    assert_eq!(config.get("Section", "Key4").unwrap(), "Four");

    assert_eq!(
        config.writes(),
        "[Section]
Key1=Value1
Key2=Value Two
Key3=this is a haiku
    spread across separate lines

    a single value
Key4=Four
"
    );

    Ok(())
}
