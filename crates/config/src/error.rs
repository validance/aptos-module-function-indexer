use std::env::VarError;
use std::fmt::{Display, Formatter};
use thiserror::Error;
use url::ParseError;

#[derive(Debug, Error)]
pub enum Error {
    VarError(VarError),
    UrlParseError(ParseError),
}

impl From<VarError> for Error {
    fn from(e: VarError) -> Self {
        Self::VarError(e)
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Self::UrlParseError(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
