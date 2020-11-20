// Holds all information about parsing record types in solar

use crate::{generics::*, identifier::Identifier, types::Type, Parse, Span, StrParse};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    multi::many1,
    sequence::preceded,
};

pub struct Structure<'a> {
    pub pos: Span<'a>,
    pub public: bool,
    pub name: Identifier<'a>,
    pub generics: GenericHeader<'a>,
    pub fields: EnumOrStructFields<'a>,
}

impl<'a> Parse<'a> for Structure<'a> {
    fn parse(s: Span<'a>) -> nom::IResult<Span<'a>, Self> {
        use crate::keyword::{key_pub, key_type};
        // pub
        let (rest, public) = opt(key_pub)(s)?;
        let public = public.is_some();
        // type Node
        let (rest, name) = preceded(key_type, Identifier::parse_ws)(rest)?;
        let pos = name.pos;
        // T
        let (rest, generics) = GenericHeader::parse_ws(rest)?;
        // - value: T
        // - next: Node T
        let (rest, fields) = EnumOrStructFields::parse_ws(rest)?;

        Ok((
            rest,
            Structure {
                pos,
                public,
                name,
                generics,
                fields,
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parsing_entire_structs_works() {
        let input = "type Node T - value T - next Node T";
        // let expected = {
        //     let name: Identifier = "Node".must_parse();
        //     let generics = "T".must_parse();
        //     let fields = "- value T - next Node T".must_parse();

        //     Structure {
        //         pos: name.pos,
        //         name,
        //         generics,
        //         fields,
        //         public: false,
        //     }
        // };

        let output: Structure = input.must_parse();
        assert_eq!(output.public, false);
        assert_eq!(&output.name.name, "Node");
    }
}

pub enum EnumOrStructFields<'a> {
    Enum(EnumFields<'a>),
    Struct(StructFields<'a>),
}

impl<'a> Parse<'a> for EnumOrStructFields<'a> {
    fn parse(s: Span<'a>) -> nom::IResult<Span<'a>, Self> {
        alt((
            map(EnumFields::parse, EnumOrStructFields::Enum),
            map(StructFields::parse, EnumOrStructFields::Struct),
        ))(s)
    }
}

pub struct EnumFields<'a> {
    pub pos: Span<'a>,
    pub states: Vec<EnumField<'a>>,
}

impl<'a> Parse<'a> for EnumFields<'a> {
    fn parse(s: Span<'a>) -> nom::IResult<Span<'a>, Self> {
        let (rest, states) = many1(EnumField::parse_ws)(s)?;

        Ok((
            rest,
            EnumFields {
                pos: states[0].pos,
                states,
            },
        ))
    }
}

pub struct EnumField<'a> {
    pub pos: Span<'a>,
    pub name: Identifier<'a>,
    // Optional value. For now can only hold one type.
    // and now name assiciated with that field
    pub value: Option<Type<'a>>,
}

impl<'a> Parse<'a> for EnumField<'a> {
    fn parse(s: Span<'a>) -> nom::IResult<Span<'a>, Self> {
        let (rest, _) = tag("|")(s)?;
        let (rest, name) = Identifier::parse_ws(rest)?;
        let (rest, value) = opt(Type::parse_ws)(rest)?;

        Ok((
            rest,
            EnumField {
                pos: name.pos,
                name,
                value,
            },
        ))
    }
}

pub struct StructFields<'a> {
    pub pos: Span<'a>,
    pub fields: Vec<StructField<'a>>,
}

// Person (name= "Nils", age = 23)
// Person name="Nils", age = 23, preference = (Computer os="macOS", vendor="apple)
// Person name="Nils" age=23 preference=(Computer os="macOS" vendor="apple)
impl<'a> Parse<'a> for StructFields<'a> {
    fn parse(s: Span<'a>) -> nom::IResult<Span<'a>, Self> {
        let (rest, fields) = many1(StructField::parse)(s)?;
        let pos = fields[0].pos;

        Ok((rest, StructFields { pos, fields }))
    }
}

pub struct StructField<'a> {
    pub pos: Span<'a>,
    pub name: Identifier<'a>,
    pub value: Type<'a>,
}

impl<'a> Parse<'a> for StructField<'a> {
    fn parse(s: Span<'a>) -> nom::IResult<Span<'a>, Self> {
        let (rest, _) = tag("-")(s)?;

        let (rest, name) = Identifier::parse_ws(rest)?;

        let (rest, value) = Type::parse_ws(rest)?;

        Ok((
            rest,
            StructField {
                pos: name.pos,
                name,
                value,
            },
        ))
    }
}
