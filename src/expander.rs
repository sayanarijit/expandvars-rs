use crate::error::{Error, Result};

pub(crate) fn expand_char(i: char) -> Result {
    Ok(i.into())
}

pub(crate) fn expand_constant(i: &[u8]) -> Result {
    String::from_utf8(i.into()).map_err(Error::from)
}

pub(crate) fn expand_variable_body(i: &[u8]) -> Result {
    let key = expand_constant(i)?;
    std::env::var(key).map_err(Error::from)
}
