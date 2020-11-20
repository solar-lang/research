use crate::Span;
use nom::bytes::complete::tag;
use nom::sequence::preceded;

pub mod characters;

pub fn whitespace(s: Span) -> nom::IResult<Span, Span> {
    use nom::bytes::complete::take_while;

    take_while(characters::whitespace)(s)
}

pub fn ws<'a, T: 'a + Sized>(
    f: impl Fn(Span<'a>) -> nom::IResult<Span<'a>, T>,
) -> impl Fn(Span<'a>) -> nom::IResult<Span<'a>, T> {
    move |input| {
        let (rest, _) = whitespace(input)?;
        f(rest)
    }
}

pub fn tag_ws<'a>(s: Span<'a>) -> impl Fn(Span<'a>) -> nom::IResult<Span<'a>, Span<'a>> {
    move |input| preceded(whitespace, tag(*s))(input)
}

#[cfg(test)]
mod test {
    use super::{tag_ws, whitespace, ws};

    #[test]
    fn whitespace() {
        let input = "    hello world";
        assert_eq!(whitespace(input), Ok(("hello world", "    ")))
    }
}
