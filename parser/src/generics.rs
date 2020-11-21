use crate::identifier::Identifier;
use crate::{Parse, Span};

/// Generic type params
/// for structs
/// either Parameter
/// or (Parameter, ...)
pub struct GenericHeader<'a> {
    pub pos: Span<'a>,
    pub params: Vec<Identifier<'a>>,
}

impl<'a> Parse<'a> for GenericHeader<'a> {
    fn parse(s: crate::Span<'a>) -> nom::IResult<crate::Span<'a>, Self> {
        use crate::util::tag_ws;
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
            |params| GenericHeader {
                // TODO this is wrong. The span needs to hold until the end of parameters
                pos: params[0].pos,
                params,
            },
        )(s)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::StrParse;
    #[test]
    fn generic_headers_parse() {
        let input = "S";
        let result: GenericHeader = input.must_parse();

        assert_eq!(result.params[0].name, "S");
    }
}
