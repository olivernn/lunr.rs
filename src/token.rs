use erased_serde::Serialize;

use std::convert::From;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::fmt;

#[derive(Debug)]
pub struct Tokens(Vec<Token>);

impl Tokens {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<String> for Tokens {
    fn from(text: String) -> Tokens {
        let tokens = text.split_whitespace().map(Token::new).collect();

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

pub type Term = String;
pub type Metadata = Box<Serialize>;

pub struct Token {
    pub term: Term,
    pub metadata: HashMap<String, Metadata>,
}

impl Token {
    fn new<S: Into<String>>(term: S) -> Token {
        Token {
            term: term.into(),
            metadata: HashMap::new(),
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: include a list of metadata keys
        write!(f, "Token({})", self.term)
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        self.term == other.term
    }
}

impl Eq for Token {}

impl Ord for Token {
    fn cmp(&self, other: &Token) -> Ordering {
        self.term.cmp(&other.term)
    }
}

impl PartialOrd for Token {
    fn partial_cmp(&self, other: &Token) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Token {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.term.hash(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizes_string() {
        let tokens: Tokens = "foo bar baz".into();
        let mut iter = tokens.into_iter().map(|t| t.term);

        assert_eq!(Some(String::from("foo")), iter.next());
        assert_eq!(Some(String::from("bar")), iter.next());
        assert_eq!(Some(String::from("baz")), iter.next());

        assert!(iter.next().is_none());
    }

    #[test]
    fn token_metadata() {
        let mut token = Token::new("foo");
        token.metadata.insert("string".into(), Box::new("string"));
        token.metadata.insert("number".into(), Box::new(123));
        token.metadata.insert("vec".into(), Box::new(vec![1, 2]));
    }
}
