use crate::util::ws;
use crate::Span;

pub trait Parse
where
    Self: Sized,
{
    fn parse(s: Span) -> nom::IResult<Span, Self>;
    fn parse_ws(s: Span) -> nom::IResult<Span, Self> {
        let (_, rest) = ws(s)?;
        Self::parse(s)
    }
}
