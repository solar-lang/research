use crate::Span;

pub fn ws(s: Span) -> nom::IResult<Span, Span> {
    use nom::bytes::complete::take_while;

    take_while(|c| c == ' ' || c == '\r' || c == '\t' || c == '\n')(s)
}
