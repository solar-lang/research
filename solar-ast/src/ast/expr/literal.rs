use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    character::complete::char,
    combinator::{map, opt, recognize, value},
    sequence::{preceded, tuple},
};

use crate::{ast::*, parse::*, util::*};
use expr::StringLiteral;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Literal<'a> {
    Bool { span: &'a str, value: bool },
    Int(Int<'a>),
    Float(&'a str),
    StringLiteral(StringLiteral<'a>),
}

#[cfg(test)]
mod literal_tests {
    use super::*;

    #[test]
    fn bool_true() {
        assert_eq!(
            Literal::parse("true "),
            Ok((
                " ",
                Literal::Bool {
                    span: "true",
                    value: true
                }
            ))
        );
    }

    #[test]
    fn bool_false() {
        assert_eq!(
            Literal::parse("false "),
            Ok((
                " ",
                Literal::Bool {
                    span: "false",
                    value: false
                }
            ))
        );
    }
}

impl<'a> Parse<'a> for Literal<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        alt((
            parse_boolean,
            map(Int::parse, Literal::Int),
            map(StringLiteral::parse, Literal::StringLiteral),
            map(parse_float, Literal::Float),
        ))(input)
    }
}

fn parse_boolean(input: &str) -> Res<Literal> {
    use keywords::{False, True};
    let t = map(True::parse, |True { span }| Literal::Bool {
        span,
        value: true,
    });
    let f = map(False::parse, |False { span }| Literal::Bool {
        span,
        value: false,
    });

    alt((t, f))(input)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Int<'a> {
    pub span: &'a str,
    pub radix: u8,
    pub digits: &'a str,
    // Int | Int64 | Int32 | Int16 | Int8
    // Uint | Uint64 | Uint32 | Uint16 | Uint8
    pub type_suffix: Option<IntTypeSuffix>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum IntTypeSuffix {
    // Integer types are 64 bits per default
    Int,
    Int32,
    Int16,
    Int8,
    Uint,
    Uint32,
    Uint16,
    Uint8,
}

fn number(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn parse_type_suffix(input: &str) -> Res<Option<IntTypeSuffix>> {
    use IntTypeSuffix::*;
    opt(alt((
        value(Int, tag("Int")),
        value(Int, tag("Int64")),
        value(Int32, tag("Int32")),
        value(Int16, tag("Int16")),
        value(Int8, tag("Int8")),
        value(Uint, tag("Uint")),
        value(Uint, tag("Uint64")),
        value(Uint32, tag("Uint32")),
        value(Uint16, tag("Uint16")),
        value(Uint8, tag("Uint8")),
    )))(input)
}

/// parses a hexadecimal number
fn hex(i: &str) -> Res<Int> {
    let (rest, digits) = preceded(
        tag("0x"),
        take_while1(|c| number(c) || c >= 'A' && c <= 'F' || c >= 'a' && c <= 'f'),
    )(i)?;
    let (rest, type_suffix) = parse_type_suffix(rest)?;
    let span = unsafe { from_to(i, rest) };

    Ok((
        rest,
        Int {
            span,
            radix: 16,
            digits,
            type_suffix,
        },
    ))
}

/// parses an octal number
fn oct(i: &str) -> Res<Int> {
    let (rest, digits) = preceded(tag("0o"), take_while1(|c| c >= '0' && c <= '7'))(i)?;
    let (rest, type_suffix) = parse_type_suffix(rest)?;
    let span = unsafe { from_to(i, rest) };

    Ok((
        rest,
        Int {
            span,
            radix: 8,
            digits,
            type_suffix,
        },
    ))
}

/// parses an binary number
fn bin(i: &str) -> Res<Int> {
    let (rest, digits) = preceded(tag("0b"), take_while1(|c| c == '0' || c == '1'))(i)?;
    let (rest, type_suffix) = parse_type_suffix(rest)?;
    let span = unsafe { from_to(i, rest) };

    Ok((
        rest,
        Int {
            span,
            radix: 2,
            digits,
            type_suffix,
        },
    ))
}

/// parses a decimal
fn dec(i: &str) -> Res<Int> {
    let (rest, digits) = take_while1(number)(i)?;
    let (rest, type_suffix) = parse_type_suffix(rest)?;
    let span = unsafe { from_to(i, rest) };

    Ok((
        rest,
        Int {
            span,
            radix: 10,
            digits,
            type_suffix,
        },
    ))
}

impl<'a> Parse<'a> for Int<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        alt((hex, oct, bin, dec))(input)
    }
}

fn parse_float<'a>(input: &'a str) -> Res<'a, &'a str> {
    let neg = opt(char('-'));
    let num = take_while1(number);
    let dot = char('.');
    // may be zero length, e.g. [1., 2., 3.]
    let second_num = take_while(number);

    let exp = opt(tuple((char('e'), opt(char('-')), take_while1(number))));

    recognize(tuple((neg, num, dot, second_num, exp)))(input)
}

#[cfg(test)]
mod float_tests {
    use super::*;

    #[test]
    fn float_value() {
        let inputs = ["3.14", "314.e-2", "-1.", "-1.0", "-1.0e20"];
        for input in inputs.iter() {
            let (rest, _) = parse_float(&input).unwrap();
            assert_eq!(rest, "");
        }
    }
}
