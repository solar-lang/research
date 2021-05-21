pub mod full;
pub use full::FullExpression;
type BFE<'a> = Box<FullExpression<'a>>;

use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded},
};

use crate::ast::expr::when::When;
use crate::ast::identifier::{FullIdentifier, Identifier};
use crate::{ast::*, parse::*, util::*};

/// Expressions
/// The main element of the solar language

//  pub span: &'a str,
pub enum Expression<'a> {
    FunctionCall(FunctionCall<'a>),
    Value(Value<'a>),
}

impl<'a> Parse<'a> for Expression<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        alt((
            map(FunctionCall::parse, Expression::FunctionCall),
            map(Value::parse, Expression::Value),
        ))(input)
    }
}

pub enum Value<'a> {
    Litaral(Literal<'a>),
    FullIdentifier(FullIdentifier<'a>),
    Closure(Closure<'a>),
    Array(Array<'a>),
    Abs(Abs<'a>),
    Tuple(Tuple<'a>),
    When(When<'a>),
    BlockExpression(BlockExpression<'a>),
}

struct Abs<'a> {
    pub span: &'a str,
    pub expr: FullExpression<'a>,
}

impl<'a> Parse<'a> for Abs<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, expr) = delimited(
            keywords::Abs::parse,
            FullExpression::parse_ws,
            keywords::Abs::parse_ws,
        )(input)?;
        let span = unsafe { from_to(input, rest) };

        Ok((rest, Abs { span, expr }))
    }
}

// may as well just be some parenthesis
struct Tuple<'a> {
    pub span: &'a str,
    pub values: Vec<FullExpression<'a>>,
}

impl<'a> Parse<'a> for Tuple<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, values) = delimited(
            keywords::ParenOpen::parse,
            many0(FullExpression::parse_ws),
            keywords::ParenClose::parse_ws,
        )(input)?;
        let span = unsafe { from_to(input, rest) };

        Ok((rest, Tuple { span, values }))
    }
}

pub struct Array<'a> {
    pub span: &'a str,
    pub values: Vec<FullExpression<'a>>,
}

impl<'a> Parse<'a> for Array<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, values) = delimited(
            keywords::BracketOpen::parse,
            many0(FullExpression::parse_ws),
            keywords::BracketClose::parse_ws,
        )(input)?;
        let span = unsafe { from_to(input, rest) };

        Ok((rest, Array { span, values }))
    }
}

pub struct FunctionCall<'a> {
    pub span: &'a str,
    // Note: may as well be a variable
    // Note: may be field access. Currently there is no distinction
    pub function_name: FullIdentifier<'a>,
    pub args: Vec<FunctionArg<'a>>,
}

impl<'a> Parse<'a> for FunctionCall<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        todo!()
    }
}

pub struct FunctionArg<'a> {
    pub name: Identifier<'a>,
    pub value: Value<'a>,
}

pub enum Literal<'a> {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(StringLiteral<'a>),
}

pub struct StringLiteral<'a> {
    span: &'a str,
    value: String,
}

impl<'a> Parse<'a> for StringLiteral<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        unimplemented!();
    }
}

// NOTE: Quite complicated, expect iterative changes
mod when {
    use super::*;

    pub enum ParenGuard<'a> {
        Bool(bool),
        Int(i64),
        Float(f64),
        String(StringLiteral<'a>),
        VariableBinding(Identifier<'a>),
        Paren(Guard<'a>),
    }

    pub struct ArrayGuard<'a> {
        pub span: &'a str,
        pub subguards: Vec<Guard<'a>>,
        pub rest: Option<Identifier<'a>>,
    }

    pub struct ObjectGuard<'a> {
        pub span: &'a str,
        pub struct_identifier: FullIdentifier<'a>,
        pub fields: Vec<(Identifier<'a>, ParenGuard<'a>)>,
    }

    pub struct TupleGuard<'a> {
        pub span: &'a str,
        pub values: Vec<Guard<'a>>,
    }

    pub enum Guard<'a> {
        Literal(Literal<'a>),
        ObjectGuard(ObjectGuard<'a>),
        ArrayGuard(ArrayGuard<'a>),
        TupleGuard(TupleGuard<'a>),
        VariableBinding(Identifier<'a>),
    }

    pub struct When<'a> {
        span: &'a str,
        branches: Vec<Branch<'a>>,
        else_clause: Option<FullExpression<'a>>,
    }

    pub struct Branch<'a> {
        is: Guard<'a>,
        then: FullExpression<'a>,
    }
}

pub struct Closure<'a> {
    pub span: &'a str,
    pub arguments: ClosureArgs<'a>,
    pub body: Box<Expression<'a>>,
}

pub enum ClosureArgs<'a> {
    Single(Identifier<'a>),
    Multiple(Identifier<'a>),
}

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
        let (rest, then) = BlockExpression::parse_ws(input)?;

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
        let (rest, body) = BlockExpression::parse_ws(input)?;

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

pub struct Loop<'a> {
    pub span: &'a str,
    pub body: BlockExpression<'a>,
}

impl<'a> Parse<'a> for Loop<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        // loop
        let (rest, _) = keywords::Loop::parse(input)?;
        // {<expr> ...}
        let (rest, body) = BlockExpression::parse_ws(input)?;

        let span = unsafe { from_to(input, rest) };

        Ok((rest, Loop { span, body }))
    }
}

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

    derive_tests!(Return, return_expr, ["return", "return 7", "return None"]);
    derive_tests!(If, if_expr, ["if true do {}", "if !true do { print x }"]);
    derive_tests!(Loop, loop_expr, ["loop {}"]);
    derive_tests!(Abs, abs_expr, ["|x|", "|[1, 2, 3]|"]);
    derive_tests!(Let, let_expr, ["let x = tag n"]);
}
