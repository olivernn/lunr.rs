use token::Token;

use unicode_segmentation::{UnicodeSegmentation, UnicodeWords};

pub struct Tokenizer<'a> {
    text: &'a str,
    words: UnicodeWords<'a>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(text: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            text: text,
            words: text.unicode_words(),
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.words.next().map(|word| {
            let begin = self.text.as_ptr().offset_to(word.as_ptr());
            let length = word.len();
            let position = (begin, length);

            let mut token = Token::new(word);
            token.metadata.insert(String::from("position"), Box::new(position));
            token
        })
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_json;
    use super::*;

    #[test]
    fn tokenizes_string() {
        let text = "The quick (\"brown\") fox can't jump 32.3 feet, right?";
        let tokens: Vec<Token> = Tokenizer::new(text).collect();

        let token_0 = &tokens[0];
        assert_eq!(token_0.term, "The");
        assert_eq!(serde_json::to_string(&token_0.metadata).unwrap(),
                   "{\"position\":[0,3]}");

        let token_1 = &tokens[1];
        assert_eq!(token_1.term, "quick");
        assert_eq!(serde_json::to_string(&token_1.metadata).unwrap(),
                   "{\"position\":[4,5]}");

        let token_2 = &tokens[2];
        assert_eq!(token_2.term, "brown");
        assert_eq!(serde_json::to_string(&token_2.metadata).unwrap(),
                   "{\"position\":[12,5]}");
    }

}
