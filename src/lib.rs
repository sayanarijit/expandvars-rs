#![feature(thread_id_value)]

mod error;
mod expander;
mod parser;

pub fn expand(input: &str) -> String {
    parser::parse(input.as_bytes()).unwrap().1.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant() {
        assert_eq!(expand("foo").as_str(), "foo");
    }
}
