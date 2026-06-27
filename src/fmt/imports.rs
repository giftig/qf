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

fn gen_rust(term: &str, text: &str) -> String {
    let r = Regex::new(r#"^\s*use\s+([\w:]+::)?[^:]+;$"#).unwrap();
    let prefix = r.replace(text, "$1").into_owned();

    format!("use {}{};", &prefix, term)
}

fn gen_go(term: &str, text: &str) -> Result<String> {
    let r = Regex::new(&r#"^.*"([^"]*{})""#.replace("{}", term)).unwrap();
    let full_term = &r.captures(text)
        .ok_or(FormatError::Pattern(format!("failed to find [{term}] in [{text}]")))?[1];

    Ok(format!(r#"import "{}""#, full_term))
}

/// Only supports "import { A, B } from 'foo.js'" style formats; depending on flavour js may also
/// use require, which is not currently supported.
fn gen_js(term: &str, text: &str) -> Result<String> {
    let r = Regex::new(r#"^import (.+) from (.+)$"#).unwrap();
    let captures = r.captures(text)
        .ok_or(FormatError::Pattern(format!("failed to find [{term}] in [{text}]")))?;

    // This could be either { Foo, Bar, Baz } or just Foo, or both like Foo, { Bar, Baz }
    // To determine which syntax to use in the import we need to determine where our term appears
    let block = &captures[1];
    let file = &captures[2];

    let braced_pattern = Regex::new(r#"\{(.+)\}"#).unwrap();
    let is_braced = braced_pattern.captures(block)
        .map(|cap| cap[1].contains(term))
        .unwrap_or(false);

    let mut out_term = term.to_string();
    if is_braced {
        out_term = format!("{{ {term} }}");
    }

    Ok(format!(r#"import {out_term} from {file}"#))
}

pub(super) fn generate_import(h: &Hit) -> Result<String> {
    match h.lang {
        DetectedLanguage::Go => gen_go(&h.term, &h.text),
        DetectedLanguage::Python => Ok(gen_py(&h.term, &h.text)),
        DetectedLanguage::Rust => Ok(gen_rust(&h.term, &h.text)),
        DetectedLanguage::Scala => Ok(gen_scala(&h.term, &h.text)),
        DetectedLanguage::Js => gen_js(&h.term, &h.text),
        _ => Err(FormatError::UnsupportedLanguage),
    }
}
