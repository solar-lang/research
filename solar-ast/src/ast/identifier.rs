use crate::*;
use solar_tokenizer::Token;

const SAMPLE_IDENTIFIER: &'static str = "anything";

pub struct FullIdentifier<'a> {
    pub tokens: &'a [Token<'a>],
    pub value: Vec<Identifier<'a>>,
}

impl<'a> Parse<'a> for FullIdentifier<'a> {
    fn parse(tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), error::Error<'a>>;
}

pub struct Identifier<'a> {
    pub tokens: &'a [Token<'a>],
    pub value: &'a str,
}

impl<'a> Parse<'a> for Identifier<'a> {
    fn parse(tokens: &'a [Token<'a>]) -> Res<'a, Self> {
        if tokens.len() == 0 {
            return Err(Error::eof()
                .recoverable()
                .expected(vec![Token::Identifier(SAMPLE_IDENTIFIER)]));
        }

        if let Token::Identifier(value) = tokens[0] {
            let ident = Identifier {
                tokens: &tokens[..1],
                value,
            };

            return Ok((ident, &tokens[..1]));
        }

        Err(Error::at_token(tokens[0])
            .recoverable()
            .expected(vec![Token::Identifier(SAMPLE_IDENTIFIER)]))
    }
}

pub fn is_keyword(word: &str) -> bool {
    [
        "lib", "in", "let", "and", "or", "when", "when", "is", "then", "else", "return", "loop",
        "break", "next", "set", "func", "function", "use", "type",
    ]
    .contains(&word)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn keyword_recognition() {
        assert!(is_keyword("let"));
        assert!(is_keyword("in"));
        assert!(is_keyword("for"));
        assert!(is_keyword("use"));
        assert!(is_keyword("type"));
        assert!(!is_keyword("x"));
        assert!(!is_keyword("y"));
        assert!(!is_keyword("point"));
    }
}
