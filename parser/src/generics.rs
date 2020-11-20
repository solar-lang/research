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
        use crate::util::{tag_ws, ws};
        use nom::branch::alt;
        use nom::bytes::complete::tag;
        use nom::combinator::map;
        use nom::multi::separated_list;
        use nom::sequence::delimited;

        let ident_list = separated_list(tag_ws(","), Identifier::parse_ws);

        map(
            alt((
                map(Identifier::parse, |v| vec![v]),
                delimited(tag("("), ident_list, tag_ws(")")),
            )),
            |parameters| GenericHeader { parameters },
        )(s)
    }
}
