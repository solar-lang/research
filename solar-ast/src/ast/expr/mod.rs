mod block;
pub mod full;
pub use block::BlockExpression;
pub use full::FullExpression;
type BFE<'a> = Box<FullExpression<'a>>;

use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::{many0, many1, separated_list0},
    sequence::{delimited, pair, preceded},
};

use crate::ast::expr::when::When;
use crate::ast::identifier::{FullIdentifier, Identifier};
use crate::{ast::*, parse::*, util::*};
use type_signature::TypeSignature;

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
        let (rest, _) = keywords::StringStart::parse(input)?;
        todo!()
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
    pub arguments: ClosureArgsKind<'a>,
    pub body: Box<Expression<'a>>,
}

impl<'a> Parse<'a> for Closure<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, arguments) = ClosureArgsKind::parse(input)?;
        // =>
        let (rest, _ ) = keywords::FatArrow::parse_ws(rest)?;
        let (rest, body) = map(Expression::parse_ws, Box::new)(rest)?;

        let span = unsafe { from_to(input, rest) };

        Ok((
            rest,
            Closure {
                span,
                arguments,
                body,
            },
        ))
    }
}

pub enum ClosureArgsKind<'a> {
    SingleArgForm(Identifier<'a>),
    NormalForm(ClosureArgs<'a>),
}

impl<'a> Parse<'a> for ClosureArgsKind<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        alt((
            map(Identifier::parse, ClosureArgsKind::SingleArgForm),
            map(ClosureArgs::parse, ClosureArgsKind::NormalForm),
        ))(input)
    }
}

pub struct ClosureArgs<'a> {
    pub span: &'a str,
    pub args: Vec<(Identifier<'a>, Option<TypeSignature<'a>>)>,
}

impl<'a> Parse<'a> for ClosureArgs<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        use keywords::*;
        let (rest, args) = delimited(
            ParenOpen::parse,
            separated_list0(
                Comma::parse_ws,
                pair(Identifier::parse_ws, opt(TypeSignature::parse_ws)),
            ),
            ParenClose::parse_ws,
        )(input)?;

        let span = unsafe { from_to(input, rest) };
        Ok((rest, ClosureArgs { span, args }))
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
    derive_tests!(Abs, abs_expr, ["|x|", "|[1, 2, 3]|"]);
    derive_tests!(ClosureArgsKind, closure_arguments, [ "(x)", "x", "(x, y)", "(x Float, y Float)", "(x Float, y Float, info)" ]);
}
