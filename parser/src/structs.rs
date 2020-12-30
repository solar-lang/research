// Holds all information about parsing record types in solar

use crate::{generics::*, identifier::Identifier, types::Type, util::to_failure, Parse, Span, Token};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    multi::many1,
};

#[derive(Clone, Debug)]
pub struct Structure<'a> {
    pub public: bool,
    pub name: Token<'a, Identifier<'a>>,
    pub generics: Token<'a, GenericHeader<'a>>,
    pub fields: EnumOrStructFields<'a>,
}

impl<'a> Parse<'a> for Structure<'a> {
    fn parse_direct(s: Span<'a>) -> nom::IResult<Span<'a>, Self> {
        use crate::keyword::{key_pub, key_type};

        // pub
        let (rest, public) = opt(key_pub)(s)?;
        let public = public.is_some();

        // type
        let (rest, _) = key_type(rest)?;

        // after this all errors are non recoverable
        // because no other token may be recognized

        // Node
        let (rest, name) = Identifier::parse_ws(rest).map_err(to_failure)?;
        // T
        let (rest, generics) = GenericHeader::parse_ws(rest).map_err(to_failure)?;
        // - value: T
        // - next: Node T
        let (rest, fields) = EnumOrStructFields::parse_direct(rest).map_err(to_failure)?;

        Ok((
            rest,
            Structure {
                public,
                name,
                generics,
                fields,
            },
        ))
    }
}

#[derive(Clone, Debug)]
pub enum EnumOrStructFields<'a> {
    Enum(Vec<Token<'a, EnumField<'a>>>),
    Struct(Vec<Token<'a, StructField<'a>>>),
}

impl<'a> Parse<'a> for EnumOrStructFields<'a> {
    fn parse_direct(s: Span<'a>) -> nom::IResult<Span<'a>, Self> {
        alt((
            map(many1(EnumField::parse_ws), EnumOrStructFields::Enum),
            map(many1(StructField::parse_ws), EnumOrStructFields::Struct),
        ))(s)
    }
}
#[derive(Clone, Debug)]
pub struct EnumField<'a> {
    pub name: Token<'a, Identifier<'a>>,
    // Optional value. For now can only hold one type.
    // and now name assiciated with that field
    pub value: Option<Token<'a, Type<'a>>>,
}

impl<'a> Parse<'a> for EnumField<'a> {
    fn parse_direct(s: Span<'a>) -> nom::IResult<Span<'a>, Self> {
        let (rest, _) = tag("|")(s)?;
        let (rest, name) = Identifier::parse_ws(rest)?;
        let (rest, value) = opt(Type::parse_ws)(rest)?;

        Ok((
            rest,
            EnumField {
                name,
                value,
            },
        ))
    }
}

#[derive(Clone, Debug)]
pub struct StructField<'a> {
    pub name: Token<'a, Identifier<'a>>,
    pub value: Token<'a, Type<'a>>,
}

impl<'a> Parse<'a> for StructField<'a> {
    fn parse_direct(s: Span<'a>) -> nom::IResult<Span<'a>, Self> {
        let (rest, _) = tag("-")(s)?;

        let (rest, name) = Identifier::parse_ws(rest)?;

        let (rest, value) = Type::parse_ws(rest)?;

        Ok((
            rest,
            StructField {
                name,
                value,
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn individual_struct_field() {
        let input = Span::from(
            "
        - value Node T",
        );

        let result = StructField::parse_ws(input);
        assert!(result.is_ok());
        let (rest, _field) = result.unwrap();
        assert_eq!(*rest, "")
    }
    #[test]
    fn parsing_entire_structs_works() {
        let input = Span::from(
            "
        pub type Node T
        - value T
        - next Node T
        ",
        );

        // let expected = {
        //     let name: Identifier = "Node".must_parse_direct);
        //     let generics = "T".must_parse_direct);
        //     let fields = "- value T - next Node T".must_parse_direct);

        //     Structure {
        //         pos: name.pos,
        //         name,
        //         generics,
        //         fields,
        //         public: false,
        //     }
        // };

        let output = Structure::parse_ws(input);
        assert!(output.is_ok());
        // TODO test more extensive
    }
}
