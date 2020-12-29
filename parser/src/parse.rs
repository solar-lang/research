use crate::{Span, Token, Unused};
use nom::combinator::map;

pub trait Parse<'a>
where
    Self: Sized,
{
    fn parse(s: Span<'a>) -> nom::IResult<Span<'a>, (Span<'a>, Self)>;
    fn parse_ws(s: Span<'a>) -> nom::IResult<Span<'a>, Token<'a, Self>> {
        let (rest, preceding) = Unused::parse_many0(s).unwrap();
        map(Self::parse, |(span, content)| Token {preceding, span, content})(s)
    }
}
