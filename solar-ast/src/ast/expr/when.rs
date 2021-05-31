use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded},
};

use super::*;

/// sugar to assert fields of a struct without the
/// trouble to write (x) or (7)
/// and instead directly write down the literal
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SubfieldGuard<'a> {
    Literal(Literal<'a>),
    VariableBinding(Identifier<'a>),
    Paren(Guard<'a>),
}

impl<'a> Parse<'a> for SubfieldGuard<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        use keywords::{ParenClose, ParenOpen};
        alt((
            map(Literal::parse, SubfieldGuard::Literal),
            map(Identifier::parse, SubfieldGuard::VariableBinding),
            map(
                delimited(ParenOpen::parse, Guard::parse_ws, ParenClose::parse_ws),
                SubfieldGuard::Paren,
            ),
        ))(input)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArrayGuard<'a> {
    pub span: &'a str,
    pub subguards: Vec<Guard<'a>>,
    pub tail: Option<Identifier<'a>>,
}
impl<'a> Parse<'a> for ArrayGuard<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, _) = keywords::ParenClose::parse(input)?;
        let (rest, subguards) = joined_by(Guard::parse_ws, keywords::Comma::parse_ws)(rest)?;
        let (rest, tail) = opt(delimited(
            keywords::Comma::parse_ws,
            Identifier::parse_ws,
            keywords::Spread::parse_ws,
        ))(rest)?;
        let (rest, _) = opt(keywords::Comma::parse_ws)(rest)?;

        let span = unsafe { from_to(input, rest) };

        Ok((
            rest,
            ArrayGuard {
                span,
                subguards,
                tail,
            },
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectGuard<'a> {
    pub span: &'a str,
    pub struct_identifier: FullIdentifier<'a>,
    pub fields: Vec<(Identifier<'a>, SubfieldGuard<'a>)>,
}
impl<'a> Parse<'a> for ObjectGuard<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, struct_identifier) = FullIdentifier::parse_ws(input)?;
        let (rest, fields) = many0(pair(Identifier::parse_ws, SubfieldGuard::parse_ws))(rest)?;

        let span = unsafe { from_to(input, rest) };
        Ok((
            rest,
            ObjectGuard {
                span,
                struct_identifier,
                fields,
            },
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TupleGuard<'a> {
    pub span: &'a str,
    pub values: Vec<Guard<'a>>,
}
impl<'a> Parse<'a> for TupleGuard<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        use keywords::{ParenClose, ParenOpen};
        let (rest, values) = delimited(
            ParenOpen::parse,
            many0(Guard::parse_ws),
            ParenClose::parse_ws,
        )(input)?;

        let span = unsafe { from_to(input, rest) };

        Ok((rest, TupleGuard { span, values }))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Guard<'a> {
    Literal(Literal<'a>),
    ObjectGuard(ObjectGuard<'a>),
    ArrayGuard(ArrayGuard<'a>),
    TupleGuard(TupleGuard<'a>),
    VariableBinding(Identifier<'a>),
}
impl<'a> Parse<'a> for Guard<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        alt((
            map(Literal::parse, Guard::Literal),
            map(ObjectGuard::parse, Guard::ObjectGuard),
            map(ArrayGuard::parse, Guard::ArrayGuard),
            map(TupleGuard::parse, Guard::TupleGuard),
            map(Identifier::parse, Guard::VariableBinding),
        ))(input)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Branch<'a> {
    pub span: &'a str,
    pub guard: Guard<'a>,
    pub then: FullExpression<'a>,
}
impl<'a> Parse<'a> for Branch<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, _) = keywords::Is::parse(input)?;
        let (rest, guard) = Guard::parse_ws(rest)?;
        let (rest, _) = keywords::Then::parse_ws(rest)?;
        let (rest, then) = FullExpression::parse_ws(rest)?;

        let span = unsafe { from_to(input, rest) };

        Ok((rest, Branch { span, guard, then }))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct When<'a> {
    pub span: &'a str,
    pub condition: FullExpression<'a>,
    pub branches: Vec<Branch<'a>>,
    pub else_clause: Option<FullExpression<'a>>,
}

impl<'a> Parse<'a> for When<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, _) = keywords::When::parse(input)?;
        let (rest, condition) = FullExpression::parse_ws(rest)?;
        let (rest, branches) = many1(Branch::parse_ws)(rest)?;
        let (rest, else_clause) =
            opt(preceded(keywords::Else::parse_ws, FullExpression::parse_ws))(rest)?;

        let span = unsafe { from_to(input, rest) };

        Ok((
            rest,
            When {
                span,
                condition,
                branches,
                else_clause,
            },
        ))
    }
}
