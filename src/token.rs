use std::string::FromUtf8Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Token<'a> {
    Const(&'a [u8]),
    Var(&'a [u8]),
}

impl Token<'_> {
    pub(crate) fn expand(self) -> Result<String, FromUtf8Error> {
        match self {
            Token::Const(s) => String::from_utf8(s.into()),
            Token::Var(s) => String::from_utf8(s.into()),
        }
    }
}
