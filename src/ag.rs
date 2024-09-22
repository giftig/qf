use std::io;
use std::process::Command;
use std::string::FromUtf8Error;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AgError {
    #[error("Ag process error: {0}")]
    Process(#[from] io::Error),
    #[error("Non-utf8 output read from ag process")]
    Utf8(#[from] FromUtf8Error),
}

pub type Result<T> = std::result::Result<T, AgError>;

pub fn ag(term: &str, filenames: bool) -> Result<String> {
    let mut c = Command::new("ag");
    c.arg("-s").arg("--column");

    if filenames {
        c.arg("-g");
    }

    c.arg(&term);

    let output = c.output()?;

    Ok(String::from_utf8(output.stdout)?)
}
