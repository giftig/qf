mod imports;
#[cfg(test)]
mod tests;

use std::iter;

use thiserror::Error;

use crate::args::{Language, OutputStyle};
use crate::search::Hit;
use crate::fmt::imports::generate_import;

#[derive(Error, Debug, PartialEq)]
pub enum FormatError {
    #[error("Missing required properties: {0}")]
    MissingProperty(String),
    #[error("Unsupported language")]
    UnsupportedLanguage,
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

    pub fn write(&self, h: &Hit) -> Result<String> {
        let res = match self.style {
            OutputStyle::Auto => {
                if h.line.is_none() {
                    h.text.to_string()
                } else {
                    Self::get_coords(h)?.join(":")
                }
            }
            OutputStyle::CleanImports => generate_import(h)?,
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
