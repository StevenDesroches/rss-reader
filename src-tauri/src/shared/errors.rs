use serde::Serialize;
use std::convert::From;

pub type Result<T> = core::result::Result<T, Error>;

// Convenience macro for creating errors
#[macro_export]
macro_rules! error {
    ($kind:expr) => {
        Error::new($kind, file!(), line!(), column!())
    };
}

#[derive(Debug, Serialize)]
pub enum ErrorType {
    Custom(String),
    ReqwestBadUrl(String),
    ReqwestBadResponse(String),
    XmlBadFormat,
    XmlDeserialize(String),
    Model(String),
    Db(String),
    Entity(String),
    // Controller(String),
    // Xml(XmlDeError)
}

#[derive(Debug, Serialize)]
pub struct ErrorContext {
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, Serialize)]
pub struct Error {
    error: ErrorType,
    context: ErrorContext,
}

impl Error {
    pub fn new(error: ErrorType, file: &'static str, line: u32, column: u32) -> Self {
        Self {
            error,
            context: ErrorContext { file, line, column },
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{:?} at {}:{}:{}", self.error, self.context.file, self.context.line, self.context.column)
    }
}

impl std::error::Error for Error {}

impl From<&str> for Error {
    fn from(val: &str) -> Self {
        error!(ErrorType::Custom(val.to_string()))
    }
}
impl From<String> for Error {
    fn from(val: String) -> Self {
        error!(ErrorType::Custom(val))
    }
}
