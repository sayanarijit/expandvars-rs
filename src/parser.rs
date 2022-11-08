use crate::error::Result;
use crate::{expander as exp, Enviroment};
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

fn parse_constant<E>(env: &mut E) -> impl FnMut(&[u8]) -> IResult<&[u8], Result> {
    |i| map(take_while1(|c| c != b'$'), exp::expand_constant)(i)
}

fn parse_variable_body<E>(env: &mut E) -> impl FnMut(&[u8]) -> IResult<&[u8], Result> {
    |i| map(take_while1(is_variable_name), exp::expand_variable_body)(i)
}

fn parse_braced_variable_body<E>(
    env: &mut E,
) -> impl FnMut(&[u8]) -> IResult<&[u8], Result> + '_ {
    |i| delimited(char('{'), parse_variable_body(env), char('}'))(i)
}

fn parse_dollar<E>(env: &mut E) -> impl FnMut(&[u8]) -> IResult<&[u8], Result> {
    |i| map(char('$'), exp::expand_char)(i)
}

fn parse_variable<E>(env: &mut E) -> impl FnMut(&[u8]) -> IResult<&[u8], Result> + '_ {
    |i| {
        preceded(
            char('$'),
            alt((parse_braced_variable_body(env), parse_variable_body(env))),
        )(i)
    }
}

fn parse_fragment<E>(env: &mut E) -> impl FnMut(&[u8]) -> IResult<&[u8], Result> + '_
where
    E: Enviroment,
{
    |i| alt((parse_variable(env), parse_constant(env), parse_dollar(env)))(i)
}

pub(crate) fn parse<E>(env: &mut E) -> impl FnMut(&[u8]) -> IResult<&[u8], Result> + '_
where
    E: Enviroment,
{
    |i| {
        if i.is_empty() {
            IResult::Ok((i, Result::Ok(String::new())))
        } else {
            fold_many1(
                parse_fragment(env),
                || Result::Ok(String::new()),
                |tokens, tok| {
                    let mut tokens = tokens?;
                    tokens.push_str(&tok?);
                    Result::Ok(tokens)
                },
            )(i)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::{FakeEnv, ProcessEnv};

    // #[test]
    // fn test_parse_constant() {
    //     assert_eq!(
    //         parse_constant(ProcessEnv)(b"foo.bar")
    //             .unwrap()
    //             .1
    //             .unwrap()
    //             .as_str(),
    //         "foo.bar"
    //     );
    // }

    // #[test]
    // fn test_parse_variable() {
    //     let var = ScopedEnv::new("value");
    //     assert_eq!(
    //         parse_variable(format!("${var}").as_bytes())
    //             .unwrap()
    //             .1
    //             .unwrap()
    //             .as_str(),
    //         "value"
    //     );
    // }
    //
    // #[test]
    // fn test_parse_braced_variable_body() {
    //     let var = ScopedEnv::new("value");
    //     assert_eq!(
    //         parse_braced_variable_body(format!("{{{var}}}").as_bytes())
    //             .unwrap()
    //             .1
    //             .unwrap()
    //             .as_str(),
    //         "value"
    //     );
    // }
    //
    // #[test]
    // fn test_dollar() {
    //     assert_eq!(parse_dollar(b"$").unwrap().1.unwrap(), "$".to_string());
    // }
    //
    // #[test]
    // fn test_parse_fragment() {
    //     let var = ScopedEnv::new("value");
    //
    //     assert_eq!(parse_fragment(b"foo").unwrap().1.unwrap().as_str(), "foo");
    //
    //     assert_eq!(
    //         parse_fragment(format!("${var}").as_bytes())
    //             .unwrap()
    //             .1
    //             .unwrap()
    //             .as_str(),
    //         "value"
    //     );
    // }

    #[test]
    fn test_parse_combo() {
        let mut env = FakeEnv::default();
        env.set("var", "value");

        assert_eq!(
            parse(&mut ProcessEnv)("foo$var.foo.${var}$".as_bytes())
                .unwrap()
                .1
                .unwrap()
                .as_str(),
            "foovalue.foo.value$"
        );
    }
}
