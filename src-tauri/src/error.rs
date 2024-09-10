use serde::Serialize;
use std::convert::From;

pub type Result<T> = core::result::Result<T, Error>;

// #[derive(Debug)]
// pub struct XmlDeError(pub quick_xml::DeError);
// impl Serialize for XmlDeError {
//     fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         let mut state = serializer.serialize_struct("DeError", 1)?;
//         state.serialize_field("error", &self.0.to_string())?;
//         state.end()
//     }
// }

#[derive(Debug, Serialize)]
pub enum Error {
    Custom(String),
    ReqwestBadUrl(String),
    ReqwestBadResponse(String),
    XmlBadFormat,
    XmlDeserialize(String),
    // Xml(XmlDeError)
}

// impl Error {
//     pub fn custom(val: impl std::fmt::Display) -> Self {
//         Self::Custom(val.to_string())
//     }
// }

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<&str> for Error {
    fn from(val: &str) -> Self {
        Self::Custom(val.to_string())
    }
}
impl From<String> for Error {
    fn from(val: String) -> Self {
        Self::Custom(val)
    }
}
