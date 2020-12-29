use crate::identifier::Identifier;
use crate::{Parse, Span, Token};

/// Generic type params
/// for structs
/// either Parameter
/// or (Parameter, ...)
#[derive(Clone, Debug)]
pub struct GenericHeader<'a> {
    pub params: Vec<Token<'a, Identifier<'a>>>,
}

impl<'a> Parse<'a> for GenericHeader<'a> {
    fn parse(s: crate::Span<'a>) -> nom::IResult<Span<'a>, (Span<'a>, Self)> {
        use crate::util::tag_ws;
        use nom::bytes::complete::tag;
        use nom::combinator::map;
        use nom::multi::separated_list;
        use nom::sequence::delimited;

        if let Ok((rest, (pos, ident))) = Identifier::parse(s) {
            let ident_token = Token::located(pos, ident);
            return Ok((
                rest,
                (
                    pos,
                    GenericHeader {
                        params: vec![ident_token],
                    },
                ),
            ));
        }

        // Note that this allows for trailing commata
        let ident_list = separated_list(tag_ws(","), Identifier::parse_ws);

        map(delimited(tag("("), ident_list, tag_ws(")")), |params| {
            // TODO this is wrong. The span needs to hold until the end of parameters
            let pos = params[0].pos;
            (pos, GenericHeader { params })
        })(s)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn generic_headers_parse() {
        let input = Span::from("S");
        let result = GenericHeader::parse(input);
        assert!(result.is_ok());
        let result = result.unwrap().1;
        assert_eq!(result.params[0].name, "S");
    }
}
