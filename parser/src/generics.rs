use crate::identifier::Identifier;
use crate::Parse;

/// Generic type parameters
/// for structs
/// either Parameter
/// or (Parameter, ...)
pub struct GenericHeader<'a> {
    parameters: Vec<Identifier<'a>>,
}

impl<'a> Parse<'a> for GenericHeader<'a> {
    fn parse(s: crate::Span<'a>) -> nom::IResult<crate::Span<'a>, Self> {
        use crate::util::characters::whitespace;
        use nom::branch::alt;
        use nom::bytes::complete::tag;
        use nom::sequence::delimited;

        alt((delimited()))
    }
}
