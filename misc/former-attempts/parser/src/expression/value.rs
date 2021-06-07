use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while, take_while1, take_while_m_n},
    combinator::map,
    multi::many1,
    number::complete::recognize_float,
    sequence::preceded,
};

use super::Expression;

use crate::{
    util::characters::{digit10, digit16, digit2, digit8},
    Parse, Span, Token,
};

#[derive(Clone, Debug)]
pub enum Value<'a> {
    Integer(IntegerType<'a>),
    Float(FloatType<'a>),
    String(StringType<'a>),
}

#[derive(Clone, Debug)]
pub struct FloatType<'a> {
    pub span: Span<'a>,
}

impl<'a> Parse<'a> for FloatType<'a> {
    fn parse_direct(s: Span<'a>) -> nom::IResult<Span<'a>, Self> {
        map(recognize_float, |span| FloatType { span })(s)
    }
}

#[derive(Clone, Debug)]
pub struct IntegerType<'a> {
    pub span: Span<'a>,
}

impl<'a> Parse<'a> for IntegerType<'a> {
    fn parse_direct(s: Span<'a>) -> nom::IResult<Span<'a>, Self> {
        let recognize_int = alt((
            take_while1(digit10),
            preceded(tag("0x"), take_while1(digit16)),
            preceded(tag("0o"), take_while1(digit8)),
            preceded(tag("0b"), take_while1(digit2)),
        ));
        map(recognize_int, |span| IntegerType { span })(s)
    }
}
