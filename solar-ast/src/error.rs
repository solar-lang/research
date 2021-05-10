use solar_tokenizer::Token;

/// Error originated while parsing tokens into ast tree
pub struct Error<'a> {
    /// Token causing the error to arise
    pub cause: Token<'a>,
    /// Tokens that would have circumvented this error
    pub expected: Option<Vec<Token<'a>>>,
    /// Wether parsing can recover after this branch of decisions failed.
    /// false per default
    recoverable: bool,
}

impl<'a> Error<'a> {
    pub fn at_token(cause: Token<'a>) -> Self {
        Error {
            cause,
            expected: None,
            recoverable: false,
        }
    }

    pub fn recoverable(self) -> Self {
        Error { recoverable: true, ..self}
    }

    pub fn expected(self, tokens: Vec<Token<'a>>) -> Self {
        // doesn't really makes sense, that expected field may already be filled
        assert_eq!(self.expected, None);
        Error { expected: Some(tokens), ..self}
    }
}

/// Error type originating while parsing
pub type Res<'a, T> = Result<(T, &'a[Token<'a>]), Error<'a>>;
