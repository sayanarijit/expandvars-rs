use crate::error::Result;
use crate::expander as exp;
use nom::branch::alt;
use nom::bytes::complete::take_while1;
use nom::character::complete::char;
use nom::character::is_alphanumeric;
use nom::combinator::map;
use nom::multi::fold_many1;
use nom::sequence::preceded;
use nom::IResult;

fn parse_constant(i: &[u8]) -> IResult<&[u8], Result> {
    map(take_while1(|c| c != b'$'), exp::expand_constant)(i)
}

fn parse_variable_body(i: &[u8]) -> IResult<&[u8], Result> {
    map(
        take_while1(|c| c == b'_' || is_alphanumeric(c)),
        exp::expand_variable_body,
    )(i)
}

fn parse_dollar(i: &[u8]) -> IResult<&[u8], Result> {
    map(char('$'), exp::expand_char)(i)
}

fn parse_variable(i: &[u8]) -> IResult<&[u8], Result> {
    preceded(char('$'), parse_variable_body)(i)
}

fn parse_fragment(i: &[u8]) -> IResult<&[u8], Result> {
    alt((parse_variable, parse_constant, parse_dollar))(i)
}

pub(crate) fn parse(i: &[u8]) -> IResult<&[u8], Result> {
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

#[cfg(test)]
mod tests {

    use std::{
        fmt::Display,
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::*;

    struct ScopedEnv(String);

    impl ScopedEnv {
        fn new(value: &str) -> Self {
            let tid = std::thread::current().id().as_u64();
            let ts = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_micros();

            let name = format!("v{}t{}", tid, ts);
            std::env::set_var(&name, value);
            Self(name)
        }
    }

    impl Display for ScopedEnv {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl<'a> Drop for ScopedEnv {
        fn drop(&mut self) {
            std::env::remove_var(&self.0);
        }
    }

    #[test]
    fn test_parse_constant() {
        assert_eq!(
            parse_constant(b"foo.bar").unwrap().1.unwrap(),
            "foo.bar".to_string()
        );
    }

    #[test]
    fn test_parse_variable() {
        let var = ScopedEnv::new("value");
        assert_eq!(
            parse_variable(format!("${var}").as_bytes())
                .unwrap()
                .1
                .unwrap(),
            "value".to_string()
        );
    }

    #[test]
    fn test_dollar() {
        assert_eq!(parse_dollar(b"$").unwrap().1.unwrap(), "$".to_string());
    }

    #[test]
    fn test_parse_fragment() {
        let var = ScopedEnv::new("value");

        assert_eq!(
            parse_fragment(b"foo").unwrap().1.unwrap(),
            "foo".to_string()
        );

        assert_eq!(
            parse_fragment(format!("${var}").as_bytes())
                .unwrap()
                .1
                .unwrap(),
            "value".to_string()
        );
    }

    #[test]
    fn test_parse_combo() {
        let var = ScopedEnv::new("value");
        assert_eq!(
            parse(format!("foo${var}.foo.${var}$").as_bytes())
                .unwrap()
                .1
                .unwrap(),
            "foovalue.foo.value$"
        );
    }
}
