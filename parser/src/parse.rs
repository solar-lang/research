use crate::util::whitespace;
use crate::Span;

pub trait Parse<'a>
where
    Self: Sized,
{
    fn parse(s: Span<'a>) -> nom::IResult<Span<'a>, Self>;
    fn parse_ws(s: Span<'a>) -> nom::IResult<Span<'a>, Self> {
        let (_, rest) = whitespace(s)?;
        Self::parse(rest)
    }
}
