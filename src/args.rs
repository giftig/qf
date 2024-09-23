#[cfg(test)]
mod tests;

use clap::{Parser, ValueEnum};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ArgError {
    #[error("Cannot use output style = import-errors without search mode = import")]
    IllegalCleanImports,
    #[error("With search mode = file, output style must be auto")]
    IllegalFileOutputMode,
}

type Result<T> = std::result::Result<T, ArgError>;

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum SearchMode {
    AllUsage,
    Class,
    File,
    Function,
    Import,
}

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum OutputStyle {
    Auto,
    Coords,
    CleanImports,
    Quickfix,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Language {
    Auto,
    Js,
    Python,
    Rust,
    Scala,
}

/// Find definitions, imports, or general uses of symbols in code and output their
/// locations in a way which enables easily jumping to or summarising these definitions.
/// Best used in conjunction with a vim plugin.
///
/// In general, the search type is the full name of a class or method, unless you're
/// searching for filenames.
#[derive(Debug, Parser)]
#[command(name = "qf")]
#[command(version = "2.0")]
pub(super) struct Args {
    /// What to search for, default is all usages. You can search for:
    ///   - classes: including traits / objects / structs etc.
    ///   - files: just find filenames matching the term
    ///   - functions: including methods. def / fn / function etc.
    ///   - imports: find examples of the given term being imported
    #[arg(value_enum, short, long, default_value_t=SearchMode::AllUsage, verbatim_doc_comment)]
    pub mode: SearchMode,

    /// Specify how the output should be presented; these options are mostly aimed at helping text
    /// editors like vim jump to or present the locations. clean-imports will write you a new
    /// import based on found uses.
    #[arg(value_enum, short, long, default_value_t=OutputStyle::Auto)]
    pub output_style: OutputStyle,

    /// Provide a language hint. This is especially useful with --output-style clean-imports
    #[arg(value_enum, long, default_value_t=Language::Auto, help = "Provide a language hint")]
    pub lang: Language,

    /// Provide only the first hit
    #[arg(short = '1', long)]
    pub first_hit: bool,

    /// List resulting filenames only
    #[arg(short, long)]
    pub list: bool,

    /// Symbol to search for
    #[arg()]
    pub term: String,
}

impl Args {
    /// Check for illegal argument combinations and report any errors so we can panic early
    pub fn validate(&self) -> Result<()> {
        if self.output_style == OutputStyle::CleanImports && self.mode != SearchMode::Import {
            return Err(ArgError::IllegalCleanImports);
        }

        if self.mode == SearchMode::File && self.output_style != OutputStyle::Auto {
            return Err(ArgError::IllegalFileOutputMode);
        }

        Ok(())
    }
}
