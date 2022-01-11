use std::fmt;
use std::result;

/// A crate private constructor for `Error`.
pub fn new_error(kind: ErrorKind) -> Error {
    Error(Box::new(kind))
}

/// A type alias for `Result<T, overseerd::Error>`.
pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error(Box<ErrorKind>);

impl Error {
    /// Return the specific type of this error.
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }

    /// Unwrap this error into its underlying type.
    pub fn into_kind(self) -> ErrorKind {
        *self.0
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum ErrorKind {
    ConfigReadingError(std::io::Error),
    ConfigParsingError(serde_yaml::Error),
    MissingConfigError(String)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            ErrorKind::ConfigReadingError(ref err) => write!(f, "Config reading error: {}", err),
            ErrorKind::ConfigParsingError(ref err) => write!(f, "Config parsing error: {}", err),
            ErrorKind::MissingConfigError(ref err) => write!(f, "MissingConfig error: {}", err),
        }
    }
}
