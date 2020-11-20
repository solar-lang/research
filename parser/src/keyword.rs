use crate::util::whitespace;
use crate::Span;
///
/// Keywords
///
/// Solar contains several keywords.
use nom::IResult;

pub fn is_keyword(input: &str) -> bool {
    // some are not used yet
    vec![
        "let", "set", "get", "for", "while", "if", "else", "then", "function", "return", "break",
        "match", "type", "is", "or", "in", "const", "pub", "async",
    ]
    .contains(&input)
}

fn key(word: &'static str) -> impl Fn(Span) -> IResult<Span, Span> {
    use nom::character::complete::one_of;
    use nom::combinator::opt;
    use nom::{bytes::complete::tag, sequence::delimited};

    move |s| delimited(whitespace, tag(word), opt(one_of(" \n\r\t")))(s)
}

pub fn key_pub(s: Span) -> IResult<Span, Span> {
    key("pub")(s)
}

pub fn key_type(s: Span) -> IResult<Span, Span> {
    key("type")(s)
}

pub fn key_get(s: Span) -> IResult<Span, Span> {
    key("get")(s)
}

pub fn key_set(s: Span) -> IResult<Span, Span> {
    key("set")(s)
}

pub fn key_function(s: Span) -> IResult<Span, Span> {
    key("function")(s)
}

pub fn key_let(s: Span) -> IResult<Span, Span> {
    key("let")(s)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_keywords() {
        let result = key_function("function".into());
        assert!(result.is_ok());
        let result = key_function("let".into());
        assert!(result.is_err());
    }
}
