use crate::token::Token;
use nom::bytes::complete::take;
use nom::bytes::complete::take_while1;
use nom::character::is_alphanumeric;
use nom::IResult;

fn constant(i: &[u8]) -> IResult<&[u8], Token> {
    let (i, res) = take_while1(|c| c != b'$')(i)?;
    Ok((i, Token::Const(res)))
}

fn variable(i: &[u8]) -> IResult<&[u8], Token> {
    let (i, s) = take(1usize)(i)?;
    if i.len() == 0 {
        return Ok((i, Token::Const(s)));
    }

    let (i, res) = take_while1(|c| c == b'_' || is_alphanumeric(c))(i)?;
    Ok((i, Token::Var(res)))
}

pub(crate) fn parse(mut i: &[u8]) -> IResult<&[u8], Vec<Token>> {
    let mut tokens = vec![];

    while !i.is_empty() {
        let fun = if i.first() == Some(&b'$') {
            variable
        } else {
            constant
        };

        let (i_, tok) = fun(i)?;
        tokens.push(tok);
        i = i_;
    }

    Ok((i, tokens))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_constant() {
        assert_eq!(parse(b"foo.bar").unwrap().1, vec![Token::Const(b"foo.bar")]);
    }

    #[test]
    fn test_parse_variable() {
        assert_eq!(parse(b"$foo").unwrap().1, vec![Token::Var(b"foo")]);
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
                Token::Const(b"$"),
            ]
        );
    }
}
