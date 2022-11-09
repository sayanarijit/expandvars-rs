use crate::env::{Enviroment, ProcessEnv};
use crate::error::Error;
use crate::token::Token;
use nom::branch::alt;
use nom::bytes::complete::take_while1;
use nom::character::complete::char;
use nom::character::is_alphanumeric;
use nom::combinator::map;
use nom::multi::fold_many1;
use nom::sequence::{delimited, preceded};
use nom::IResult;

fn is_variable_name(c: u8) -> bool {
    c == b'_' || is_alphanumeric(c)
}

fn parse_constant(i: &[u8]) -> IResult<&[u8], Token> {
    map(take_while1(|c| c != b'$'), Token::Const)(i)
}

fn parse_pid(i: &[u8]) -> IResult<&[u8], Token> {
    map(char('$'), |_| Token::Pid)(i)
}

fn parse_variable_body(i: &[u8]) -> IResult<&[u8], Token> {
    alt((parse_pid, map(take_while1(is_variable_name), Token::Var)))(i)
}

fn parse_braced_variable_body(i: &[u8]) -> IResult<&[u8], Token> {
    delimited(char('{'), parse_variable_body, char('}'))(i)
}

fn parse_dollar(i: &[u8]) -> IResult<&[u8], Token> {
    map(char('$'), Token::Char)(i)
}

fn parse_variable(i: &[u8]) -> IResult<&[u8], Token> {
    preceded(
        char('$'),
        alt((parse_braced_variable_body, parse_variable_body, parse_pid)),
    )(i)
}

fn parse_fragment(i: &[u8]) -> IResult<&[u8], Token> {
    alt((parse_variable, parse_constant, parse_dollar))(i)
}

pub(crate) fn parse_with<'a, E>(
    env: &mut E,
    i: &'a [u8],
) -> IResult<&'a [u8], Result<String, Error>>
where
    E: Enviroment,
{
    if i.is_empty() {
        IResult::Ok((i, Ok(String::new())))
    } else {
        fold_many1(
            parse_fragment,
            || Ok(String::new()),
            |tokens, tok| {
                let mut tokens = tokens?;
                tokens.push_str(&tok.expand_with(env)?);
                Ok(tokens)
            },
        )(i)
    }
}

pub(crate) fn parse(i: &[u8]) -> IResult<&[u8], Result<String, Error>> {
    parse_with(&mut ProcessEnv, i)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::env::FakeEnv;

    #[test]
    fn test_parse_constant() {
        assert_eq!(
            parse_constant(b"foo.bar").unwrap().1,
            Token::Const(b"foo.bar")
        );
    }

    #[test]
    fn test_parse_variable() {
        assert_eq!(parse_variable(b"$var").unwrap().1, Token::Var(b"var"));
    }

    #[test]
    fn test_parse_braced_variable_body() {
        assert_eq!(
            parse_braced_variable_body(b"{var}").unwrap().1,
            Token::Var(b"var")
        );
    }

    #[test]
    fn test_dollar() {
        assert_eq!(parse_dollar(b"$").unwrap().1, Token::Char('$'));
    }

    #[test]
    fn test_parse_fragment() {
        assert_eq!(parse_fragment(b"foo").unwrap().1, Token::Const(b"foo"));
        assert_eq!(parse_fragment(b"$var").unwrap().1, Token::Var(b"var"));
    }

    #[test]
    fn test_parse_combo() {
        let mut env = FakeEnv::empty();
        env.set("var", "value");

        assert_eq!(
            parse_with(&mut env, b"foo$var.foo.${var}$")
                .unwrap()
                .1
                .unwrap(),
            "foovalue.foo.value$"
        );
    }
}
