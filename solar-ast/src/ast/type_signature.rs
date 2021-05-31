use crate::ast::identifier::FullIdentifier;

use crate::{ast::*, parse::*, util::*};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeSignature<'a> {
    pub span: &'a str,
    pub type_kind: TypeKind<'a>,
    pub return_type: Option<Box<TypeSignature<'a>>>,
}

impl<'a> Parse<'a> for TypeSignature<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        use keywords::ThinArrow;
        use nom::sequence::preceded;

        let (rest, type_kind) = TypeKind::parse(input)?;

        if let Ok((rest, return_type)) =
            preceded(ThinArrow::parse_ws, TypeSignature::parse_ws)(rest)
        {
            let span = unsafe { from_to(input, rest) };
            let return_type = Some(Box::new(return_type));
            return Ok((
                rest,
                TypeSignature {
                    span,
                    type_kind,
                    return_type,
                },
            ));
        }

        let span = unsafe { from_to(input, rest) };

        Ok((
            rest,
            TypeSignature {
                span,
                type_kind,
                return_type: None,
            },
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TypeKind<'a> {
    DirectType(DirectType<'a>),
    TupleType(TupleType<'a>),
    VectorType(VectorType<'a>),
}

impl<'a> Parse<'a> for TypeKind<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        use nom::{branch::alt, combinator::map};

        alt((
            map(DirectType::parse, TypeKind::DirectType),
            map(TupleType::parse, TypeKind::TupleType),
            map(VectorType::parse, TypeKind::VectorType),
        ))(input)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirectType<'a> {
    pub span: &'a str,
    // may contain (relative) path to type
    pub full_identifier: FullIdentifier<'a>,
    pub generic_argument: Option<Box<TypeSignature<'a>>>,
}

impl<'a> Parse<'a> for DirectType<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, full_identifier) = identifier::FullIdentifier::parse(input)?;

        // if there is some generic argument next
        if let Ok((rest, generic_argument)) = TypeSignature::parse_ws(rest) {
            let span = unsafe { from_to(input, rest) };
            let generic_argument = Some(Box::new(generic_argument));
            return Ok((
                rest,
                DirectType {
                    span,
                    full_identifier,
                    generic_argument,
                },
            ));
        }

        let span = unsafe { from_to(input, rest) };
        Ok((
            rest,
            DirectType {
                span,
                full_identifier,
                generic_argument: None,
            },
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TupleType<'a> {
    pub span: &'a str,
    // may contain (relative) path to type
    pub types: Vec<TypeSignature<'a>>,
}

impl<'a> Parse<'a> for TupleType<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        use keywords::*;
        use nom::{multi::separated_list0, sequence::delimited};

        let (rest, types) = delimited(
            ParenOpen::parse,
            separated_list0(Comma::parse_ws, TypeSignature::parse_ws),
            ParenClose::parse_ws,
        )(input)?;

        let span = unsafe { from_to(input, rest) };

        Ok((rest, TupleType { span, types }))
    }
}

// Just the same as Vec <generic_argument>
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VectorType<'a> {
    pub span: &'a str,
    pub generic_argument: Box<TypeSignature<'a>>,
}

impl<'a> Parse<'a> for VectorType<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        use keywords::*;
        // parse []
        let (rest, _l) = BracketOpen::parse(input)?;
        let (rest, _r) = BracketClose::parse_ws(rest)?;

        let (rest, generic_argument) = TypeSignature::parse_ws(rest)?;

        let span = unsafe { from_to(input, rest) };
        let generic_argument = Box::new(generic_argument);

        Ok((
            rest,
            VectorType {
                span,
                generic_argument,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn types0() {
        let input = "std.List";
        let res = TypeSignature::parse(input);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().1.span, input);
    }

    #[test]
    fn types1() {
        let input = "std.List String";
        let res = TypeSignature::parse(input);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().1.span, input);
    }

    #[test]
    fn types2() {
        let input = "Map (String, Json)";
        let res = TypeSignature::parse(input);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().1.span, input);
    }

    #[test]
    fn types3() {
        let input = "List (Map (String, Json))";
        let res = TypeSignature::parse(input);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().1.span, input);
    }

    #[test]
    fn tuples() {
        let inputs = ["()", "(Abc)", "(Abc.defg)", "( x, y, z)"];

        for input in inputs.iter() {
            let res = TupleType::parse(input);
            assert!(res.is_ok());
            let res = res.unwrap();
            assert_eq!(&res.1.span, input);
            assert_eq!(res.0, "");
        }
    }

    #[test]
    fn types4() {
        let input = "[]Float";
        let res = TypeSignature::parse(input);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().1.span, input);
    }

    #[test]
    fn types5() {
        let input = "[][]Float";
        let res = TypeSignature::parse(input);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().1.span, input);
    }

    #[test]
    fn vectors() {
        let inputs = ["[]N", "[]Abc", "[](Abc.defg)", "[][][][]x"];

        for input in inputs.iter() {
            let res = VectorType::parse(input);
            assert!(res.is_ok());
            let res = res.unwrap();
            assert_eq!(&res.1.span, input);
            assert_eq!(res.0, "");
        }
    }

    #[test]
    fn types6() {
        let input = "List List String";
        let res = TypeSignature::parse(input);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().1.span, input);
    }

    #[test]
    fn types7() {
        let input = "List Uint8 -> T";
        let res = TypeSignature::parse(input);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().1.span, input);
    }

    #[test]
    fn types8() {
        let input = "Float -> Float -> String";
        let res = TypeSignature::parse(input);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().1.span, input);
    }

    #[test]
    fn types9() {
        let input = "Float -> (Float -> String)";
        let res = TypeSignature::parse(input);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().1.span, input);
    }

    #[test]
    fn types10() {
        let input = "(Float -> Float) -> String";
        let res = TypeSignature::parse(input);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().1.span, input);
    }
}
