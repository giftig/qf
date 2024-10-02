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

#[derive(Clone, Debug)]
pub struct Ag {
    extra_args: Vec<String>
}

impl Ag {
    pub fn new(extra_args: Vec<String>) -> Ag {
        Ag { extra_args: extra_args }
    }
    pub fn default() -> Ag {
        Ag { extra_args: vec![] }
    }

    pub fn ag<S: AsRef<str>>(&self, term: &str, filenames: bool, extra_args: &[S]) -> Result<String> {
        let mut c = Command::new("ag");
        c.arg("-s").arg("--column");

        for arg in &self.extra_args {
            c.arg(&arg);
        }
        for arg in extra_args {
            c.arg(arg.as_ref());
        }

        if filenames {
            c.arg("-g");
        }

        c.arg(&term);

        let output = c.output()?;

        Ok(String::from_utf8(output.stdout)?)
    }
}
