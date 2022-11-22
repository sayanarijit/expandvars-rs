use env::{Enviroment, ProcessEnv};

pub mod env;
pub mod error;
mod parser;
mod token;

#[cfg(test)]
mod tests;

pub fn expand(input: &str) -> error::Result {
    expand_with(&mut ProcessEnv, input)
}

pub fn expand_with<E>(env: &mut E, input: &str) -> error::Result
where
    E: Enviroment,
{
    let tokens = parser::parse(input.as_bytes()).unwrap().1?;
    let mut res = String::new();
    for tok in tokens {
        res.push_str(&tok.expand_with(env)?);
    }
    Ok(res)
}
