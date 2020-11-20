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

// utility trait so one can just call .parse on any string
// and the annoted type get's parsed (or a panic occurs)
pub trait StrParse<T> {
    fn must_parse(self) -> T;
}

impl<'a, T: Parse<'a>> StrParse<T> for &'a str {
    fn must_parse(self) -> T {
        T::parse_ws(Span::from(self)).expect("parsing failed").1
    }
}
