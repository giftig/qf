use super::*;

use crate::args::{Language, OutputStyle};
use crate::search::Hit;

/// A typical hit for a filename search, no coordinates
fn filename_hit(filename: &str) -> Hit {
    Hit {
        term: filename.to_string(),
        filename: filename.to_string(),
        line: None,
        col: None,
        text: filename.to_string()
    }
}

/// A typical hit for a normal term search, coordinates included
fn term_hit(term: &str, text: &str) -> Hit {
    Hit {
        term: term.to_string(),
        filename: "Example.scala".to_string(),
        line: Some(1337),
        col: Some(66),
        text: text.to_string()
    }
}

#[test]
/// Use coordinates format if no specific format specified
fn auto_coord_fmt() {
    let formatter = HitFormatter::new(&OutputStyle::Auto, &Language::Auto);
    let hit = term_hit("Example", "class Example");

    let expected = "Example.scala:1337:66".to_string();
    let actual = formatter.write(&hit);

    assert_eq!(actual, Ok(expected));
}

#[test]
/// Show text only with auto format if no line/col info (filename search only)
fn auto_coord_filename_fmt() {
    let formatter = HitFormatter::new(&OutputStyle::Auto, &Language::Auto);
    let hit = filename_hit("Example.scala");

    let expected = "Example.scala".to_string();
    let actual = formatter.write(&hit);

    assert_eq!(actual, Ok(expected));
}

#[test]
/// Show coord format if specified
fn coord_fmt() {
    let formatter = HitFormatter::new(&OutputStyle::Coords, &Language::Auto);
    let hit = term_hit("Example", "class Example");

    let expected = "Example.scala:1337:66".to_string();
    let actual = formatter.write(&hit);

    assert_eq!(actual, Ok(expected));
}

#[test]
/// Error if coord format specified but coords not found
fn coord_fmt_no_coords() {
    let formatter = HitFormatter::new(&OutputStyle::Coords, &Language::Auto);
    let hit = filename_hit("Example.scala");

    let expected = FormatError::MissingProperty("line number".to_string());
    let actual = formatter.write(&hit);

    assert_eq!(actual, Err(expected));
}

#[test]
/// Show quickfix format if specified
fn quickfix_fmt() {
    let formatter = HitFormatter::new(&OutputStyle::Quickfix, &Language::Auto);
    let hit = term_hit("Example", "class Example");

    let expected = "Example.scala:1337:66:class Example".to_string();
    let actual = formatter.write(&hit);

    assert_eq!(actual, Ok(expected));
}

#[test]
/// Error if quickfix format specified but coords not found
fn quickfix_fmt_no_coords() {
    let formatter = HitFormatter::new(&OutputStyle::Quickfix, &Language::Auto);
    let hit = filename_hit("Example.scala");

    let expected = FormatError::MissingProperty("line number".to_string());
    let actual = formatter.write(&hit);

    assert_eq!(actual, Err(expected));
}

#[test]
fn import_fmt_scala_single() {
    let formatter = HitFormatter::new(&OutputStyle::CleanImports, &Language::Auto);
    let hit = term_hit("Potato", "import com.example.foo.bar.Potato");

    let expected = "import com.example.foo.bar.Potato".to_string();
    let actual = formatter.write(&hit);

    assert_eq!(actual, Ok(expected));
}

#[test]
fn import_fmt_scala_first_in_group() {
    let formatter = HitFormatter::new(&OutputStyle::CleanImports, &Language::Auto);
    let hit = term_hit("Potato", "import com.example.foo.bar.{Potato, Zucchini}");

    let expected = "import com.example.foo.bar.Potato".to_string();
    let actual = formatter.write(&hit);

    assert_eq!(actual, Ok(expected));
}

#[test]
fn import_fmt_scala_last_in_group() {
    let formatter = HitFormatter::new(&OutputStyle::CleanImports, &Language::Auto);
    let hit = term_hit("Potato", "import com.example.foo.bar.{Cucumber, Parsnip, Potato}");

    let expected = "import com.example.foo.bar.Potato".to_string();
    let actual = formatter.write(&hit);

    assert_eq!(actual, Ok(expected));
}

#[test]
fn import_fmt_scala_mid_group() {
    let formatter = HitFormatter::new(&OutputStyle::CleanImports, &Language::Auto);
    let hit = term_hit("Potato", "import com.example.foo.bar.{Cucumber, Parsnip, Potato, Zucchini}");

    let expected = "import com.example.foo.bar.Potato".to_string();
    let actual = formatter.write(&hit);

    assert_eq!(actual, Ok(expected));
}
