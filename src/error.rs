//! Markdown -> LaTeX conversion errors.

use std::fmt;
use std::io;
use std::error;
use std::result;
use std::ops::Deref;
use std::borrow::Cow;

/// A Markdown -> LaTeX conversion error.
#[derive(Debug)]
pub struct Error {
    /// The error message.
    message: Cow<'static, str>,
    /// The underlying error, if any.
    cause: Option<Box<error::Error>>,
}

impl Error {
    /// Instantiate an `Error` using a message.
    pub fn new<S>(message: S) -> Self
        where S: Into<Cow<'static, str>>
    {
        Error {
            message: message.into(),
            cause: None,
        }
    }

    /// Instantiate an `Error` using a message and a cause.
    pub fn with_cause<S, E>(message: S, cause: E) -> Self
        where S: Into<Cow<'static, str>>,
              E: error::Error + 'static,
    {
        Error {
            message: message.into(),
            cause: Some(Box::new(cause)),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.cause {
            Some(ref cause) => write!(f, "{}: {}", self.message, cause),
            None => f.write_str(&self.message),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&error::Error> {
        self.cause.as_ref().map(Deref::deref)
    }
}

impl From<fmt::Error> for Error {
    fn from(error: fmt::Error) -> Self {
        Self::with_cause("Formatting error", error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::with_cause("I/O error", error)
    }
}

/// A result type that contains an MDCCC `Error`.
pub type Result<T> = result::Result<T, Error>;
