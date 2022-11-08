use crate::error::Result;
use crate::expander as exp;
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

fn parse_constant(i: &[u8]) -> IResult<&[u8], Result> {
    map(take_while1(|c| c != b'$'), exp::expand_constant)(i)
}

fn parse_variable_body(i: &[u8]) -> IResult<&[u8], Result> {
    map(take_while1(is_variable_name), exp::expand_variable_body)(i)
}

fn parse_braced_variable_body(i: &[u8]) -> IResult<&[u8], Result> {
    delimited(char('{'), parse_variable_body, char('}'))(i)
}

fn parse_dollar(i: &[u8]) -> IResult<&[u8], Result> {
    map(char('$'), exp::expand_char)(i)
}

fn parse_variable(i: &[u8]) -> IResult<&[u8], Result> {
    preceded(
        char('$'),
        alt((parse_braced_variable_body, parse_variable_body)),
    )(i)
}

fn parse_fragment(i: &[u8]) -> IResult<&[u8], Result> {
    alt((parse_variable, parse_constant, parse_dollar))(i)
}

pub(crate) fn parse(i: &[u8]) -> IResult<&[u8], Result> {
    if i.is_empty() {
        IResult::Ok((i, Result::Ok(String::new())))
    } else {
        fold_many1(
            parse_fragment,
            || Result::Ok(String::new()),
            |tokens, tok| {
                let mut tokens = tokens?;
                tokens.push_str(&tok?);
                Result::Ok(tokens)
            },
        )(i)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use scoped_rand_env::Env;

    #[test]
    fn test_parse_constant() {
        assert_eq!(
            parse_constant(b"foo.bar").unwrap().1.unwrap().as_str(),
            "foo.bar"
        );
    }

    #[test]
    fn test_parse_variable() {
        let var = Env::new("value");
        assert_eq!(
            parse_variable(format!("${var}").as_bytes())
                .unwrap()
                .1
                .unwrap()
                .as_str(),
            "value"
        );
    }

    #[test]
    fn test_parse_braced_variable_body() {
        let var = Env::new("value");
        assert_eq!(
            parse_braced_variable_body(format!("{{{var}}}").as_bytes())
                .unwrap()
                .1
                .unwrap()
                .as_str(),
            "value"
        );
    }

    #[test]
    fn test_dollar() {
        assert_eq!(parse_dollar(b"$").unwrap().1.unwrap(), "$".to_string());
    }

    #[test]
    fn test_parse_fragment() {
        let var = Env::new("value");

        assert_eq!(parse_fragment(b"foo").unwrap().1.unwrap().as_str(), "foo");

        assert_eq!(
            parse_fragment(format!("${var}").as_bytes())
                .unwrap()
                .1
                .unwrap()
                .as_str(),
            "value"
        );
    }

    #[test]
    fn test_parse_combo() {
        let var = Env::new("value");
        assert_eq!(
            parse(format!("foo${var}.foo.${{{var}}}$").as_bytes())
                .unwrap()
                .1
                .unwrap()
                .as_str(),
            "foovalue.foo.value$"
        );
    }
}
