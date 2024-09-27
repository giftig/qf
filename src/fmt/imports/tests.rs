use super::*;

use crate::search::{DetectedLanguage, Hit};

/// A basic hit for an import. Note that filename isn't important as we're
/// providing the detected language directly
fn basic_hit(term: &str, text: &str, lang: &DetectedLanguage) -> Hit {
    Hit {
        term: term.to_string(),
        filename: "arbitrary-filename.txt".to_string(),
        line: Some(1337),
        col: Some(66),
        text: text.to_string(),
        lang: lang.clone(),
    }
}

#[test]
fn gen_scala_single() {
    let hit = basic_hit(
        "Potato",
        "import com.example.foo.bar.Potato",
        &DetectedLanguage::Scala,
    );

    let expected = "import com.example.foo.bar.Potato".to_string();
    let actual = generate_import(&hit);

    assert_eq!(actual, Ok(expected));
}

#[test]
fn gen_scala_first_in_group() {
    let hit = basic_hit(
        "Potato",
        "import com.example.foo.bar.{Potato, Zucchini}",
        &DetectedLanguage::Scala,
    );

    let expected = "import com.example.foo.bar.Potato".to_string();
    let actual = generate_import(&hit);

    assert_eq!(actual, Ok(expected));
}

#[test]
fn gen_scala_last_in_group() {
    let hit = basic_hit(
        "Potato",
        "import com.example.foo.bar.{Cucumber, Parsnip, Potato}",
        &DetectedLanguage::Scala,
    );

    let expected = "import com.example.foo.bar.Potato".to_string();
    let actual = generate_import(&hit);

    assert_eq!(actual, Ok(expected));
}

#[test]
fn gen_scala_mid_group() {
    let hit = basic_hit(
        "Potato",
        "import com.example.foo.bar.{Cucumber, Parsnip, Potato, Zucchini}",
        &DetectedLanguage::Scala,
    );

    let expected = "import com.example.foo.bar.Potato".to_string();
    let actual = generate_import(&hit);

    assert_eq!(actual, Ok(expected));
}

#[test]
fn gen_python_simple_single() {
    let hit = basic_hit("my_sneks", "import my_sneks", &DetectedLanguage::Python);

    let expected = "import my_sneks".to_string();
    let actual = generate_import(&hit);

    assert_eq!(actual, Ok(expected));
}

#[test]
fn gen_python_simple_single_dotted() {
    let hit = basic_hit("my_sneks", "import stuff.my_sneks", &DetectedLanguage::Python);

    let expected = "import stuff.my_sneks".to_string();
    let actual = generate_import(&hit);

    assert_eq!(actual, Ok(expected));
}

#[test]
fn gen_python_simple_first_in_group() {
    let hit = basic_hit("sneks", "import sneks, zebras", &DetectedLanguage::Python);

    let expected = "import sneks".to_string();
    let actual = generate_import(&hit);

    assert_eq!(actual, Ok(expected));
}

#[test]
fn gen_python_simple_last_in_group() {
    let hit = basic_hit("sneks", "import mice, sneks", &DetectedLanguage::Python);

    let expected = "import sneks".to_string();
    let actual = generate_import(&hit);

    assert_eq!(actual, Ok(expected));
}

#[test]
fn gen_python_from_clause_single() {
    let hit = basic_hit("sneks", "from zoo import sneks", &DetectedLanguage::Python);

    let expected = "from zoo import sneks".to_string();
    let actual = generate_import(&hit);

    assert_eq!(actual, Ok(expected));
}

#[test]
fn gen_python_from_clause_mid_group() {
    let hit = basic_hit(
        "sneks",
        "from zoo.cages import aardvarks, sneks, zebras",
        &DetectedLanguage::Python
    );

    let expected = "from zoo.cages import sneks".to_string();
    let actual = generate_import(&hit);

    assert_eq!(actual, Ok(expected));
}

#[test]
fn gen_rust_single() {
    let hit = basic_hit("Potato", "use crate::produce::Potato;", &DetectedLanguage::Rust);

    let expected = "use crate::produce::Potato;".to_string();
    let actual = generate_import(&hit);

    assert_eq!(actual, Ok(expected));
}

#[test]
fn gen_rust_no_prefix() {
    let hit = basic_hit("stuffs", "use stuffs;", &DetectedLanguage::Rust);

    let expected = "use stuffs;".to_string();
    let actual = generate_import(&hit);

    assert_eq!(actual, Ok(expected));
}

#[test]
fn gen_rust_mid_group() {
    let hit = basic_hit("Potato", "use stuff::{Car, Potato, Sieve};", &DetectedLanguage::Rust);

    let expected = "use stuff::Potato;".to_string();
    let actual = generate_import(&hit);

    assert_eq!(actual, Ok(expected));
}

#[test]
/// Fail to generate an import if the language is unsupported
fn import_unsupported_language() {
    let hit = basic_hit("Potato", "#include<Potato.h>", &DetectedLanguage::Unknown);

    let expected = FormatError::UnsupportedLanguage;
    let actual = generate_import(&hit);

    assert_eq!(actual, Err(expected));
}
