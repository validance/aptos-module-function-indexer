pub mod module_function;
pub mod move_module;

use diesel::result::Error as DieselError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum DbError {
    #[error("Initializing inner db failed")]
    InnerDbInitFailed {},

    #[error("Client connection failed")]
    ClientConnectionFailed {},

    #[error("Diesel error")]
    DieselError(DieselError),

    #[error("Type Conversion Failed {msg}")]
    ConversionError { msg: String },
}

impl From<DieselError> for DbError {
    fn from(e: DieselError) -> Self {
        Self::DieselError(e)
    }
}
