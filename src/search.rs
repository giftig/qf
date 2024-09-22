use std::num::ParseIntError;

use thiserror::Error;

use crate::ag::{AgError, ag};
use crate::args::SearchMode;

#[derive(Error, Debug)]
pub enum SearchError {
    #[error("Ag error: {0}")]
    Ag(#[from] AgError),
    #[error("Hit fragment count error")]
    HitFragmentCount,
    #[error("Error parsing line/col from ag: {0}")]
    HitParseError(#[from] ParseIntError)
}

type Result<T> = std::result::Result<T, SearchError>;

#[derive(Clone, Debug)]
pub struct Hit {
    pub term: String,
    pub filename: String,
    pub line: Option<u64>,
    pub col: Option<u64>,
    pub text: String
}

impl Hit {
    /// During a regular search we get filename, line, col, and then the hit text
    fn parse(line: &str, term: &str) -> Result<Hit> {
        let pieces: Vec<&str> = line.split(":").collect();

        if pieces.len() < 4 {
            return Err(SearchError::HitFragmentCount);
        }

        return Ok(
            Hit {
                term: term.to_string(),
                filename: pieces[0].to_string(),
                line: Some(pieces[1].parse::<u64>()?),
                col: Some(pieces[2].parse::<u64>()?),
                text: pieces[3..].join(":")
            }
        )
    }

    /// When filenames are searched, all we get is the filename
    fn parse_filename(line: &str, term: &str) -> Result<Hit> {
        Ok(Hit { term: term.to_string(), filename: line.to_string(), line: None, col: None, text: line.to_string() })
    }
}

pub struct Search {
    mode: SearchMode,
}

impl Search {
    pub fn new(mode: &SearchMode) -> Search {
        Search { mode: mode.clone() }
    }

    /// Wrap the term in an appropriate regex depending on the search mode
    pub fn get_pattern(&self, term: &str) -> String {
        // Regex raw quote
        let raw = format!("\\Q{}\\E", term);

        let fmt = match self.mode {
            SearchMode::AllUsage | SearchMode::File =>
                "{}",
            SearchMode::Class =>
                r#"(?:class|trait|object|type|struct|impl|enum) {}\s*(?:[\[\(\{{: ]|$)"#,
            SearchMode::Function =>
                r#"(?:def|fn|function) {}[\[\(: ]"#,
            SearchMode::Import =>
                r#"(?:import|use) .*[\.\{{,: ]{}(?:[\{{\}},; ]|$)"#,
        };

        return fmt.replace("{}", &raw);
    }

    /// Perform a search for a given term, based on the search config
    pub fn search(&self, term: &str) -> Result<Vec<Hit>> {
        Ok(
            ag(&self.get_pattern(&term), self.mode == SearchMode::File)?
                .split("\n")
                .into_iter()
                .filter(|line| !line.trim().is_empty())
                .map(|line| {
                    if self.mode == SearchMode::File {
                        return Hit::parse_filename(line, term).unwrap();
                    }

                    // FIXME
                    Hit::parse(line, term).unwrap()
                })
                .collect()
        )
    }
}
