use super::{FormatError, Result};

use regex::Regex;

use crate::search::{DetectedLanguage, Hit};

pub(super) fn generate_import(h: &Hit) -> Result<String> {
    match h.lang {
        DetectedLanguage::Scala => {
            let r = Regex::new(r#"^\s*import ([^\s]+)\..*$"#).unwrap();
            let prefix = r.replace(&h.text, "$1").into_owned();

            Ok(format!("import {}.{}", &prefix, &h.term))
        },
        _ => {
            Err(FormatError::UnsupportedLanguage)
        }
    }
}
