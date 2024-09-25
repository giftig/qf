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
fn generate_import_scala_first_in_group() {
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
fn import_fmt_scala_last_in_group() {
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
fn import_fmt_scala_mid_group() {
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
/// Fail to generate an import if the language is unsupported
fn import_unsupported_language() {
    let hit = basic_hit("Potato", "#include<Potato.h>", &DetectedLanguage::Unknown);

    let expected = FormatError::UnsupportedLanguage;
    let actual = generate_import(&hit);

    assert_eq!(actual, Err(expected));
}
