use solar_tokenizer::Token;
use crate::Tokens;

impl<'a> Into<nom::Err<Error<'a>>> for Error<'a> {
    fn into(self) -> nom::Err<Error<'a>> {
        match self {
            // we'd lose information here
            // Error::TokenError(t) if t.cause.is_none() => nom::Err::Incomplete(nom::Needed::Unknown),
            Error::TokenError(t) if t.recoverable == false => nom::Err::Failure(t.into()),
            e => nom::Err::Error(e)
        }
    }
}

pub enum Error<'a> {
    TokenError(TokenError<'a>),
    NomError(nom::error::Error<Tokens<'a>>)
}

    impl<'a> From<nom::error::Error<Tokens<'a>>> for Error<'a> {
    fn from(e: nom::error::Error<Tokens<'a>>) -> Self {
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
    /// Token causing the error to arise. No token means End of input
    pub cause: Option<Token<'a>>,
    /// Tokens that would have circumvented this error
    pub expected: Option<Tokens<'a>>,
    /// Wether parsing can recover after this branch of decisions failed.
    /// false per default
    recoverable: bool,
}

impl<'a> TokenError<'a> {
    pub fn end_of_input() -> Self {
        TokenError {
            cause: None,
            expected: None,
            recoverable: false,
        }
    }

    pub fn at_token(cause: Token<'a>) -> Self {
        TokenError {
            cause: Some(cause),
            expected: None,
            recoverable: false,
        }
    }

    pub fn recoverable(self) -> Self {
        TokenError { recoverable: true, ..self}
    }

    pub fn expected(self, tokens: Tokens<'a>) -> Self {
        // doesn't really makes sense, that expected field may already be filled
        assert_eq!(self.expected, None);
        TokenError { expected: Some(tokens), ..self}
    }
}