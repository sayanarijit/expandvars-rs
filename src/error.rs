use std::{env::VarError, string::FromUtf8Error};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    FromUtf8Error(FromUtf8Error),
    VarError(VarError),
}

impl From<VarError> for Error {
    fn from(v: VarError) -> Self {
        Self::VarError(v)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(v: FromUtf8Error) -> Self {
        Self::FromUtf8Error(v)
    }
}

pub type Result = std::result::Result<String, Error>;
