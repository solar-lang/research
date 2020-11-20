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

pub fn tag_ws<'a>(s: &'a str) -> impl Fn(Span<'a>) -> nom::IResult<Span<'a>, Span<'a>> {
    move |input| preceded(whitespace, tag(s))(input)
}

#[cfg(test)]
mod test {
    use super::{tag_ws, whitespace, ws};
    use crate::Span;

    #[test]
    fn check_whitespace_combinators_work() {
        let input = Span::from("    hello world");
        let (rest, parsed) = whitespace(input).unwrap();
        assert_eq!(*rest, "hello world");
        assert_eq!(*parsed, "    ");

        let (rest, parsed) = tag_ws("hello")(input).unwrap();
        assert_eq!(*rest, " world");
        assert_eq!(*parsed, "hello");
    }
}
