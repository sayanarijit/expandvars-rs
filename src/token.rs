use crate::{env::Enviroment, error::Error};

fn to_string(bytes: &[u8]) -> Result<String, Error> {
    String::from_utf8(bytes.into()).map_err(Error::from)
}

// TODO use OsString?
fn get_value<E>(name: &[u8], env: &E) -> Result<Option<String>, Error>
where
    E: Enviroment,
{
    let key = String::from_utf8(name.into()).map_err(Error::from)?;
    let var = if let Some(val) = env.get(&key) {
        if val.is_empty() {
            None
        } else {
            Some(val.to_string_lossy().to_string())
        }
    } else {
        None
    };

    Ok(var)
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Token<'a> {
    Const(&'a [u8]),
    Var(&'a [u8]),
    Char(char),
    Pid,
    VarWithDefault(&'a [u8], Box<Token<'a>>),
}

impl Token<'_> {
    pub(crate) fn expand_with<E>(self, env: &mut E) -> Result<String, Error>
    where
        E: Enviroment,
    {
        match self {
            Token::Const(s) => to_string(s),
            Token::Char(c) => Ok(c.into()),
            Token::Pid => Ok(std::process::id().to_string()),
            Token::Var(name) => get_value(name, env).map(|v| v.unwrap_or_default()),
            Token::VarWithDefault(name, default) => {
                let default = default.expand_with(env)?;
                get_value(name, env).map(|v| v.unwrap_or(default))
            }
        }
    }
}
