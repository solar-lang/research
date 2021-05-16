use solar_tokenizer::Token;
use crate::Tokens;

pub enum Error<'a> {
    TokenError(TokenError<'a>),
    NomError(nom::Err<nom::error::Error<Tokens<'a>>>)
}

impl<'a> From<nom::Err<nom::error::Error<Tokens<'a>>>> for Error<'a> {
    fn from(e: nom::Err<nom::error::Error<Tokens<'a>>>) -> Self {
        Error::NomError(e)
    }
}

impl<'a> From<TokenError<'a>> for Error<'a> {
    fn from(e: TokenError<'a>) -> Self {
        Error::TokenError(e)
    }
}

/// Error originated while parsing tokens into ast tree
pub struct TokenError<'a> {
    /// Token causing the error to arise
    pub cause: Token<'a>,
    /// Tokens that would have circumvented this error
    pub expected: Option<Vec<Token<'a>>>,
    /// Wether parsing can recover after this branch of decisions failed.
    /// false per default
    recoverable: bool,
}

impl<'a> TokenError<'a> {
    pub fn at_token(cause: Token<'a>) -> Self {
        TokenError {
            cause,
            expected: None,
            recoverable: false,
        }
    }

    pub fn recoverable(self) -> Self {
        TokenError { recoverable: true, ..self}
    }

    pub fn expected(self, tokens: Vec<Token<'a>>) -> Self {
        // doesn't really makes sense, that expected field may already be filled
        assert_eq!(self.expected, None);
        TokenError { expected: Some(tokens), ..self}
    }
}