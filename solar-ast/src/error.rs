use solar_tokenizer::Token;

pub struct Error<'a> {
    cause: Token<'a>
}

/// Error type originating while parsing
pub type Res<'a, T> = Result<(T, &'a[Token<'a>]), Error<'a>>;
