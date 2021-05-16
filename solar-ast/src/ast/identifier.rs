use core::str;

use solar_tokenizer::Token;
use crate::{Error, Parse, Res, TokenError, Tokens};

pub struct FullIdentifier<'a> {
    pub tokens: &'a [Token<'a>],
    pub value: Vec<Identifier<'a>>,
}


#[derive(Debug)]
pub struct Identifier<'a> {
    pub tokens: &'a [Token<'a>],
    pub value: &'a str,
}

impl<'a> PartialEq<&str> for Identifier<'a> {
    fn eq(&self, other: &&str) -> bool {
        self.value == *other
    }
}

impl<'a> Parse<'a> for Identifier<'a> {
    fn parse(tokens: Tokens<'a>) -> nom::IResult<Tokens<'a>, Self, Error> {
        const EXPECTED: Tokens<'static> = &[Token::Identifier("some_identifier")];

        if tokens.len() == 0{
            return Err(
                nom::Err::Error(Error::TokenError(TokenError::end_of_input().expected(EXPECTED).recoverable()))
            );
        }

        match &tokens[0] {
            Token::Identifier(value) => Ok((&tokens[1..], Identifier {tokens: &tokens[..1], value})),
            cause => Err(TokenError::at_token(cause).expected(EXPECTED).recoverable().into()),
        }
    }
}

pub fn is_keyword(word: &str) -> bool {
    [
        "lib", "in", "let", "and", "or", "when", "when", "is", "then", "else", "return", "loop",
        "break", "next", "set", "func", "function", "use", "type", "for"
    ]
    .contains(&word)
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
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

    #[test]
    fn idents() {
        let tokens = [Token::Identifier("hello"), Token::Abs("|")];
        let res = Identifier::parse(&tokens);

        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.0.len(), 1);
        assert_eq!(res.1, "hello");
    }
}
