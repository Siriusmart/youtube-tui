use std::{error::Error, fmt::Display};

/// Errors for displaying custom string in the message bar if there is an error
#[derive(Debug)]
pub enum Errors {
    BlankError,
    StringError(String),
    StrError(&'static str),
}

impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BlankError => f.write_str("Is a blank error ¯\\_(ツ)_/¯"),
            Self::StringError(e) => f.write_str(&e),
            Self::StrError(e) => f.write_str(e),
        }
    }
}

impl Error for Errors {}
