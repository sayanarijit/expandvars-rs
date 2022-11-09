use crate::{env::Enviroment, error::Error};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Token<'a> {
    Const(&'a [u8]),
    Var(&'a [u8]),
    Char(char),
}

impl Token<'_> {
    // TODO use OsString?
    pub(crate) fn expand_with<E>(self, env: &mut E) -> Result<String, Error>
    where
        E: Enviroment,
    {
        match self {
            Token::Const(s) => String::from_utf8(s.into()).map_err(Error::from),
            Token::Var(s) => {
                let key = String::from_utf8(s.into()).map_err(Error::from)?;
                if let Some(var) = env.get(&key) {
                    Ok(var.to_string_lossy().to_string())
                } else {
                    Ok(Default::default())
                }
            }
            Token::Char(c) => Ok(c.into()),
        }
    }
}
