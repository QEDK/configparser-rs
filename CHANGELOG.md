## Changelog

- 0.1.0 (yanked)
  - First experimental version with only a public-facing load() function.
- 0.1.1
  - `configparser` module renamed to `ini`.
- 0.2.1
  - `Ini` struct is added along with file-loading, parsing and hashmap functions. Documentation is added.
- 0.2.2
  - Fixed docs.
- 0.3.0
  - Added `get()` for getting values from the map directly. Docs expanded as well.
  - Mark `ini::load()` for deprecation.
- 0.3.1
  - Updated docs.
  - All parameters now trimmed before insertion.
  - Converted `ini::load()` into a wrapper around `Ini`.
- 0.4.0
  - Changed `Ini::load()` to return an `Ok(map)` with a clone of the stored `HashMap`.
- 0.4.1
  - Fixed and added docs.
- 0.5.0 (**BETA**) (yanked)
  - Changelog added.
  - Support for value-less keys.
  - `HashMap` values are now `Option<String>` instead of `String` to denote empty values vs. no values.
  - Documentation greatly improved.
  - Syntax docs provided.
  - `new()` and `get()` methods are simplified.
- 0.5.1
  - Fixed erroneous docs

Older changelogs are preserved here, current changelog is present in [README.md](README.md).