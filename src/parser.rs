use crate::token::Token;
use nom::branch::alt;
use nom::bytes::complete::take_while1;
use nom::character::complete::char;
use nom::character::is_alphanumeric;
use nom::combinator::map;
use nom::multi::fold_many1;
use nom::sequence::preceded;
use nom::IResult;

fn parse_constant(i: &[u8]) -> IResult<&[u8], Token> {
    map(take_while1(|c| c != b'$'), Token::Const)(i)
}

fn parse_variable_body(i: &[u8]) -> IResult<&[u8], Token> {
    map(take_while1(|c| c == b'_' || is_alphanumeric(c)), Token::Var)(i)
}

fn parse_dollar(i: &[u8]) -> IResult<&[u8], Token> {
    map(char('$'), Token::Char)(i)
}

fn parse_variable(i: &[u8]) -> IResult<&[u8], Token> {
    preceded(char('$'), parse_variable_body)(i)
}

fn parse_fragment(i: &[u8]) -> IResult<&[u8], Token> {
    alt((parse_variable, parse_constant, parse_dollar))(i)
}

pub(crate) fn parse(i: &[u8]) -> IResult<&[u8], Vec<Token>> {
    fold_many1(parse_fragment, Vec::new, |mut tokens, tok| {
        tokens.push(tok);
        tokens
    })(i)
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
        assert_eq!(parse_variable(b"$foo").unwrap().1, Token::Var(b"foo"));
    }

    #[test]
    fn test_dollar() {
        assert_eq!(parse_dollar(b"$").unwrap().1, Token::Char('$'));
    }

    #[test]
    fn test_parse_fragment() {
        assert_eq!(parse_fragment(b"foo").unwrap().1, Token::Const(b"foo"));
        assert_eq!(parse_fragment(b"$foo").unwrap().1, Token::Var(b"foo"));
    }

    #[test]
    fn test_parse_combo() {
        assert_eq!(
            parse(b"foo$bar.foo.$bar$").unwrap().1,
            vec![
                Token::Const(b"foo"),
                Token::Var(b"bar"),
                Token::Const(b".foo."),
                Token::Var(b"bar"),
                Token::Char('$'),
            ]
        );
    }
}
