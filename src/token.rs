use std::convert::From;

pub struct Tokens(Vec<Token>);

impl Tokens {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<String> for Tokens {
    fn from(text: String) -> Tokens {
        let tokens = text.split_whitespace()
            .map(|s| Token { string: s.to_owned() })
            .collect();

        Tokens(tokens)
    }
}

impl<'a> From<&'a str> for Tokens {
    fn from(text: &'a str) -> Tokens {
        text.to_owned().into()
    }
}

impl IntoIterator for Tokens {
    type Item = Token;
    type IntoIter = ::std::vec::IntoIter<Token>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub struct Token {
    string: String
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizes_string() {
        let tokens: Tokens = "foo bar baz".into();
        let mut iter = tokens.into_iter().map(|t| t.string);

        assert_eq!(Some(String::from("foo")), iter.next());
        assert_eq!(Some(String::from("bar")), iter.next());
        assert_eq!(Some(String::from("baz")), iter.next());

        assert!(iter.next().is_none());
    }
}
