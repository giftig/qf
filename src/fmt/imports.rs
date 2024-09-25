#[cfg(test)]
pub mod tests;

use super::{FormatError, Result};

use regex::Regex;

use crate::search::{DetectedLanguage, Hit};

/// Generate a python import in the format of the one found
/// This should match the style used (either `import foo` or `from x import y`) but will need
/// to strip down multi-imports and renames where necessary
fn gen_py(term: &str, text: &str) -> String {
    let r = Regex::new(r#"^((?:from [\w\.]+ )?import (?:[\w\.]+\.)?).*$"#).unwrap();
    let prefix = r.replace(text, "$1").into_owned();
    format!("{}{}", &prefix, term)
}

fn gen_scala(term: &str, text: &str) -> String {
    let r = Regex::new(r#"^\s*import ([^\s]+)\..*$"#).unwrap();
    let prefix = r.replace(text, "$1").into_owned();

    format!("import {}.{}", &prefix, term)
}

pub(super) fn generate_import(h: &Hit) -> Result<String> {
    match h.lang {
        DetectedLanguage::Scala => Ok(gen_scala(&h.term, &h.text)),
        DetectedLanguage::Python => Ok(gen_py(&h.term, &h.text)),
        _ => Err(FormatError::UnsupportedLanguage),
    }
}
