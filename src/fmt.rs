#[cfg(test)]
mod tests;

use std::iter;

use regex::Regex;
use thiserror::Error;

use crate::args::{Language, OutputStyle};
use crate::search::Hit;

#[derive(Error, Debug, PartialEq)]
pub enum FormatError {
    #[error("Missing required properties: {0}")]
    MissingProperty(String),
}

type Result<T> = std::result::Result<T, FormatError>;

pub struct HitFormatter {
    style: OutputStyle,
    lang: Language,
}

impl HitFormatter {
    pub fn new(style: &OutputStyle, lang: &Language) -> HitFormatter {
        HitFormatter {
            style: style.clone(),
            lang: lang.clone(),
        }
    }

    pub fn get_coords(h: &Hit) -> Result<[String; 3]> {
        Ok([
            h.filename.to_string(),
            h.line
                .ok_or(FormatError::MissingProperty("line number".to_string()))?
                .to_string(),
            h.col
                .ok_or(FormatError::MissingProperty("col number".to_string()))?
                .to_string(),
        ])
    }

    /// Generate an import which is valid in the target language, based on found imports
    /// This will deal with removing multi-import structures where they're found
    fn generate_import(&self, h: &Hit) -> String {
        // FIXME: May need a separate struct for this, as we'll need to handle different input
        // and output formats for each language. Probably also need to store source language
        // in the Hit as that's a better source of the info.
        // This impl was taken from the python version and should (mostly) work for Scala imports
        let r = Regex::new(r#"^\s*import ([^\s]+)\..*$"#).unwrap();
        let prefix = r.replace(&h.text, "$1").into_owned();

        format!("import {}.{}", &prefix, &h.term)
    }

    pub fn write(&self, h: &Hit) -> Result<String> {
        let res = match self.style {
            OutputStyle::Auto => {
                if h.line.is_none() {
                    h.text.to_string()
                } else {
                    Self::get_coords(h)?.join(":")
                }
            }
            OutputStyle::CleanImports => self.generate_import(h),
            OutputStyle::Coords => Self::get_coords(h)?.join(":"),
            OutputStyle::Quickfix => Self::get_coords(h)?
                .into_iter()
                .chain(iter::once(h.text.to_string()))
                .collect::<Vec<_>>()
                .join(":"),
        };

        Ok(res)
    }
}
