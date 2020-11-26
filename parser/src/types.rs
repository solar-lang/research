use crate::util::{tag_ws, ws};
use crate::{identifier::Identifier, Parse, Span};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::multi::separated_list;
use nom::sequence::delimited;

use nom::multi::many1;

/// Represents a type with full generics attached.
/// e.g.
/// let x: Array String
/// let m: Map (String, Int64)
///
/// TODO not enough to represent [Int64, String] or [Float64; 4] etc.
#[derive(Clone, Debug)]
pub struct Type<'a> {
    pub pos: Span<'a>,
    // Name of type
    pub name: Identifier<'a>,

    // Generic parameters
    pub params: Vec<Type<'a>>,
}

impl<'a> Parse<'a> for Type<'a> {
    fn parse(s: Span<'a>) -> nom::IResult<Span<'a>, Self> {
        let (rest, name) = Identifier::parse(s)?;
        let pos = name.pos;

        let (rest, params) = {
            let type_list = separated_list(tag_ws(","), Type::parse_ws);

            ws(alt((
                many1(Type::parse_ws),
                delimited(tag("("), type_list, tag_ws(")")),
            )))(rest)?
        };

        // TODO pos has to include all parameters. Right now it only spans the first identifier.
        Ok((rest, Type { name, params, pos }))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parsing_simple_types() {
        let input = Span::from("Node T");
        let result = Type::parse(input);
        assert!(result.is_ok());
    }

    #[test]
    fn parsing_complex_types() {
        let input = Span::from("Map (Key, Value)");
        let result = Type::parse(input);
        assert!(result.is_ok());
    }

    #[test]
    fn parsing_complex_types2() {
        let input = Span::from("List List T");
        let result = Type::parse(input);
        assert!(result.is_ok());
    }
}
