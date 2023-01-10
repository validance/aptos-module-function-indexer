use database::models::DbError;
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    DbError(DbError),
}

impl From<DbError> for Error {
    fn from(e: DbError) -> Self {
        Self::DbError(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format_args!("{:?}", self))
    }
}
