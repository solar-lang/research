use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, pair, preceded},
};

use crate::{ast::*, parse::*, util::*};
use expr::FullExpression;
use identifier::Identifier;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlockExpression<'a> {
    pub span: &'a str,
    pub parts: Vec<BlockExpressionPart<'a>>,
}

impl<'a> Parse<'a> for BlockExpression<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        // {
        let (rest, _) = keywords::CurlyOpen::parse(input)?;
        let (rest, parts) = many0(BlockExpressionPart::parse_ws)(rest)?;
        let (rest, _) = keywords::CurlyClose::parse_ws(rest)?;

        let span = unsafe { from_to(input, rest) };

        Ok((rest, BlockExpression { span, parts }))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BlockExpressionPart<'a> {
    Let(Let<'a>),
    Return(Return<'a>),
    Break(&'a str),
    Next(&'a str),
    Loop(Loop<'a>),
    If(If<'a>),
    For(For<'a>),
    FullExpression(FullExpression<'a>),
    // ;
    Separator(&'a str),
}

impl<'a> Parse<'a> for BlockExpressionPart<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let brk = map(keywords::Break::parse, |keywords::Break { span }| {
            BlockExpressionPart::Break(span)
        });
        let next = map(keywords::Next::parse, |keywords::Next { span }| {
            BlockExpressionPart::Next(span)
        });
        let sep = map(
            keywords::SemiColon::parse,
            |keywords::SemiColon { span }| BlockExpressionPart::Separator(span),
        );

        alt((
            map(Let::parse, BlockExpressionPart::Let),
            map(Return::parse, BlockExpressionPart::Return),
            brk,
            next,
            map(Loop::parse, BlockExpressionPart::Loop),
            map(If::parse, BlockExpressionPart::If),
            map(For::parse, BlockExpressionPart::For),
            map(FullExpression::parse, BlockExpressionPart::FullExpression),
            sep,
        ))(input)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct If<'a> {
    pub span: &'a str,
    pub condition: FullExpression<'a>,
    pub then: BlockExpression<'a>,
}

impl<'a> Parse<'a> for If<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        // if
        let (rest, _) = keywords::If::parse(input)?;
        // <expr>
        let (rest, condition) = FullExpression::parse_ws(rest)?;
        // do
        let (rest, _) = keywords::Do::parse_ws(rest)?;
        // {<expr> ...}
        let (rest, then) = BlockExpression::parse_ws(rest)?;

        let span = unsafe { from_to(input, rest) };

        Ok((
            rest,
            If {
                span,
                condition,
                then,
            },
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct For<'a> {
    pub span: &'a str,
    pub variable: Identifier<'a>,
    pub over: FullExpression<'a>,
    pub body: BlockExpression<'a>,
}

impl<'a> Parse<'a> for For<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        // for
        let (rest, _) = keywords::For::parse(input)?;
        // x
        let (rest, variable) = Identifier::parse_ws(rest)?;
        // in
        let (rest, _) = keywords::In::parse_ws(rest)?;
        // list     e.g. <expr>
        let (rest, over) = FullExpression::parse_ws(rest)?;
        // do
        let (rest, _) = keywords::Do::parse_ws(rest)?;
        // {<expr> ...}
        let (rest, body) = BlockExpression::parse_ws(rest)?;

        let span = unsafe { from_to(input, rest) };

        Ok((
            rest,
            For {
                span,
                variable,
                over,
                body,
            },
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Loop<'a> {
    pub span: &'a str,
    pub body: BlockExpression<'a>,
}

impl<'a> Parse<'a> for Loop<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        // loop
        let (rest, _) = keywords::Loop::parse(input)?;
        // {<expr> ...}
        let (rest, body) = BlockExpression::parse_ws(rest)?;

        let span = unsafe { from_to(input, rest) };

        Ok((rest, Loop { span, body }))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Let<'a> {
    pub span: &'a str,
    pub identifier: Identifier<'a>,
    pub expr: FullExpression<'a>,
}

impl<'a> Parse<'a> for Let<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, (identifier, expr)) = pair(
            delimited(
                keywords::Let::parse,
                Identifier::parse_ws,
                keywords::Assign::parse_ws,
            ),
            FullExpression::parse_ws,
        )(input)?;

        let span = unsafe { from_to(input, rest) };

        Ok((
            rest,
            Let {
                span,
                identifier,
                expr,
            },
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Return<'a> {
    pub span: &'a str,
    pub value: Option<FullExpression<'a>>,
}

impl<'a> Parse<'a> for Return<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, value) =
            preceded(keywords::Return::parse, opt(FullExpression::parse_ws))(input)?;

        let span = unsafe { from_to(input, rest) };

        Ok((rest, Return { span, value }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! derive_tests {
        ($ty:ty, $testname:ident, $list:tt) => {
            #[test]
            fn $testname() {
                let input = $list;
                for i in input.iter() {
                    let (rest, _) = <$ty>::parse(i).unwrap();
                    assert_eq!(rest, "");
                }
            }
        };
    }

    #[test]
    fn if_test1() {
        let input = "if true do {something} ";
        let (rest, statement) = If::parse(input).unwrap();
        assert_eq!(
            statement,
            If {
                span: "if true do {something}",
                condition: FullExpression::parse("true").unwrap().1,
                then: BlockExpression::parse("{something}").unwrap().1,
            }
        );
        assert_eq!(rest, " ");
    }

    derive_tests!(
        Return,
        return_statement,
        ["return", "return 7", "return None"]
    );
    derive_tests!(If, if_statement, ["if !true do { print x }"]);
    derive_tests!(Loop, loop_statement, ["loop {}"]);
    derive_tests!(Let, let_statement, ["let x = tag n"]);
}
