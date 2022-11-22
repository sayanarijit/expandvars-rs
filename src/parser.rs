use crate::error::Error;
use crate::token::Token;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while1;
use nom::character::complete::char;
use nom::character::is_alphanumeric;
use nom::combinator::map;
use nom::multi::fold_many0;
use nom::sequence::{delimited, preceded, separated_pair};
use nom::IResult;

fn is_variable_name(c: u8) -> bool {
    c == b'_' || is_alphanumeric(c)
}

fn parse_constant(i: &[u8]) -> IResult<&[u8], Token> {
    map(take_while1(|c| c != b'$' && c != b'}'), Token::Const)(i)
}

fn parse_pid(i: &[u8]) -> IResult<&[u8], Token> {
    map(char('$'), |_| Token::Pid)(i)
}

fn parse_variable_name(i: &[u8]) -> IResult<&[u8], Token> {
    map(take_while1(is_variable_name), Token::Var)(i)
}

fn parse_variable_name_with_default(i: &[u8]) -> IResult<&[u8], Token> {
    map(
        separated_pair(
            take_while1(is_variable_name),
            alt((tag("-"), tag(":-"))),
            alt((parse_fragment, map(tag(""), Token::Const))),
        ),
        |(name, default)| Token::VarWithDefault(name, Box::new(default)),
    )(i)
}

fn parse_variable_body(i: &[u8]) -> IResult<&[u8], Token> {
    alt((
        parse_pid,
        parse_variable_name_with_default,
        parse_variable_name,
    ))(i)
}

fn parse_braced_variable_body(i: &[u8]) -> IResult<&[u8], Token> {
    delimited(char('{'), parse_variable_body, char('}'))(i)
}

fn parse_dollar(i: &[u8]) -> IResult<&[u8], Token> {
    map(char('$'), Token::Char)(i)
}

fn parse_closing_brace(i: &[u8]) -> IResult<&[u8], Token> {
    map(char('}'), Token::Char)(i)
}

fn parse_variable(i: &[u8]) -> IResult<&[u8], Token> {
    preceded(
        char('$'),
        alt((parse_braced_variable_body, parse_variable_body, parse_pid)),
    )(i)
}

fn parse_fragment(i: &[u8]) -> IResult<&[u8], Token> {
    alt((
        parse_variable,
        parse_constant,
        parse_dollar,
        parse_closing_brace,
    ))(i)
}

pub(crate) fn parse<'a>(
    i: &'a [u8],
) -> IResult<&'a [u8], Result<Vec<Token<'a>>, Error>> {
    fold_many0(
        parse_fragment,
        || Ok(Vec::new()),
        |tokens, tok| {
            let mut tokens = tokens?;
            tokens.push(tok);
            Ok(tokens)
        },
    )(i)
}

#[cfg(test)]
mod tests {

    use super::*;

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
    fn test_parse_variable_with_default() {
        assert_eq!(
            parse_variable_name_with_default(b"var-default").unwrap().1,
            Token::VarWithDefault(b"var", Box::new(Token::Const(b"default")))
        );

        assert_eq!(
            parse_variable_name_with_default(b"var-").unwrap().1,
            Token::VarWithDefault(b"var", Box::new(Token::Const(b"")))
        );

        assert_eq!(
            parse_variable_name_with_default(b"var:-").unwrap().1,
            Token::VarWithDefault(b"var", Box::new(Token::Const(b"")))
        );
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
    fn test_pars() {
        use Token::*;

        assert_eq!(
            parse(b"foo$var.foo.${var}}${var-}$").unwrap().1.unwrap(),
            vec![
                Const(b"foo"),
                Var(b"var"),
                Const(b".foo."),
                Var(b"var"),
                Char('}'),
                VarWithDefault(b"var", Box::new(Const(b""))),
                Char('$')
            ]
        );
    }
}
