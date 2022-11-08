mod parser;
mod token;

pub fn expand(input: &str) -> String {
    let (_, tokens) = parser::parse(input.as_bytes()).unwrap();
    let mut out = String::new();
    for tok in tokens {
        let val = tok.expand().unwrap();
        out.push_str(&val);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant() {
        assert_eq!(expand("foo").as_str(), "foo");
    }
}
