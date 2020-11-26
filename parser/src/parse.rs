use crate::util::whitespace;
use crate::Located;
use crate::Span;

pub trait Parse<'a>
where
    Self: Sized,
{
    fn parse(s: Span<'a>) -> nom::IResult<Span<'a>, Self>;
    fn parse_ws(s: Span<'a>) -> nom::IResult<Span<'a>, Self> {
        let (rest, _whitespace_characters) = whitespace(s).unwrap();
        Self::parse(rest)
    }
}
