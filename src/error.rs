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
    /// Instantiate an `Error` using a message and a cause.
    #[allow(trivial_casts)]
    pub fn new<S, E>(message: S, cause: Option<E>) -> Self
        where S: Into<Cow<'static, str>>,
              E: error::Error + 'static,
    {
        Error {
            message: message.into(),
            cause: cause.map(|e| Box::new(e) as Box<error::Error>),
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
        Self::new("Formatting error", error.into())
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::new("I/O error", error.into())
    }
}

/// A result type that contains an MDCCC `Error`.
pub type Result<T> = result::Result<T, Error>;
