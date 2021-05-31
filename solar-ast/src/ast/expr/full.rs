// full expression

use crate::ast::expr::{Expression, FunctionCall};
use crate::{ast::*, parse::*, util::*};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FullExpression<'a> {
    And(And<'a>),
    Or(Or<'a>),
    Concat(Concat<'a>),
    Add(Add<'a>),
    Subtract(Subtract<'a>),
    Multiply(Multiply<'a>),
    Divide(Divide<'a>),
    Not(Not<'a>),
    Power(Power<'a>),
    Negate(Negate<'a>),
    Sqrt(Sqrt<'a>),
    Pipe(Pipe<'a>),
    // // direct field access
    // Dot(BFE<'a>, BFE<'a>),
    Expression(Box<Expression<'a>>),
}

impl<'a> Parse<'a> for FullExpression<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        And::parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_expr() {
        let input = [
            "x + y",
            "x+y",
            "x+y+z",
            "(x+y)*7",
            "(x+y) : double",
            "n^8",
            "x",
            "n/8+9:something",
        ];

        for i in input.iter() {
            let (rest, _fe) = FullExpression::parse(i).unwrap();
            assert_eq!(rest, "");
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pipe<'a> {
    pub span: &'a str,
    pub expr: Box<Expression<'a>>,
    pub function_call: FunctionCall<'a>,
}

impl<'a> Pipe<'a> {
    fn parse(input: &'a str) -> Res<'a, FullExpression> {
        let (rest, expr) = Expression::parse(input)?;
        let expr = Box::new(expr);

        if let Ok((rest, _)) = keywords::Colon::parse_ws(rest) {
            let (rest, function_call) = FunctionCall::parse_ws(rest)?;

            let span = unsafe { from_to(input, rest) };

            return Ok((
                rest,
                Pipe {
                    span,
                    expr,
                    function_call,
                }
                .into(),
            ));
        }

        let expr = FullExpression::Expression(expr);

        Ok((rest, expr))
    }
}

impl<'a> Into<FullExpression<'a>> for Pipe<'a> {
    fn into(self) -> FullExpression<'a> {
        FullExpression::Pipe(self)
    }
}

/// create a simple AST Node
/// Used in Full Expression
macro_rules! create_ast_expr {
    ($name:ident, $separator:ty, $next_struct:ty) => {

        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $name<'a> {
            pub span: &'a str,
            pub left: Box<FullExpression<'a>>,
            pub right: Box<FullExpression<'a>>,
        }

        impl<'a> $name<'a> {
            fn parse(input: &'a str) -> Res<'a, FullExpression<'a>> {
                let (rest, left) = <$next_struct>::parse(input)?;

                if let Ok((rest, _)) = <$separator>::parse_ws(rest) {
                    let (rest, right) = <$next_struct>::parse_ws(rest)?;
                    let span = unsafe { from_to(input, rest) };
                    let left: Box<FullExpression<'a>> = Box::new(left.into());
                    let right: Box<FullExpression<'a>> = Box::new(right.into());

                    return Ok((rest, FullExpression::$name($name { span, left, right })));
                }

                Ok((rest, left.into()))
            }

            fn parse_ws(input: &'a str) -> Res<'a, FullExpression<'a>> {
                crate::parse::ws(Self::parse)(input)
            }
        }
    };
}

create_ast_expr!(And, keywords::And, Or);
create_ast_expr!(Or, keywords::Or, Concat);
create_ast_expr!(Concat, keywords::Concat, Add);
create_ast_expr!(Add, keywords::Add, Subtract);
create_ast_expr!(Subtract, keywords::Subtract, Multiply);
create_ast_expr!(Multiply, keywords::Multiply, Divide);
create_ast_expr!(Divide, keywords::Divide, Not);
create_ast_expr!(Not, keywords::Not, Power);
create_ast_expr!(Power, keywords::Power, Negate);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Negate<'a> {
    pub span: &'a str,
    pub expr: Box<Expression<'a>>,
}

impl<'a> Parse<'a> for Negate<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, _) = keywords::Minus::parse(input)?;
        let (rest, expr) = Expression::parse_ws(rest)?;

        let span = unsafe { from_to(input, rest) };
        let expr = Box::new(expr);

        Ok((rest, Negate { span, expr }))
    }
}

impl<'a> Into<FullExpression<'a>> for Negate<'a> {
    fn into(self) -> FullExpression<'a> {
        FullExpression::Negate(self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Sqrt<'a> {
    pub span: &'a str,
    pub expr: Box<Expression<'a>>,
}

impl<'a> Parse<'a> for Sqrt<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, _) = keywords::Sqrt::parse(input)?;
        let (rest, expr) = Expression::parse_ws(rest)?;

        let span = unsafe { from_to(input, rest) };
        let expr = Box::new(expr);

        Ok((rest, Sqrt { span, expr }))
    }
}

impl<'a> Into<FullExpression<'a>> for Sqrt<'a> {
    fn into(self) -> FullExpression<'a> {
        FullExpression::Sqrt(self)
    }
}
