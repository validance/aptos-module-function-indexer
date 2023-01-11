use std::env::VarError;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use thiserror::Error;
use url::ParseError;

#[derive(Debug, Error)]
pub enum Error {
    Var(VarError),
    UrlParse(ParseError),
    ParseInt(ParseIntError),
}

impl From<VarError> for Error {
    fn from(e: VarError) -> Self {
        Self::Var(e)
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Self::UrlParse(e)
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Self::ParseInt(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
