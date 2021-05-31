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

    // √x:n
    // may not be
    // √(x:n)

    // list : filter ft : map n * 3 ++ [end_elem]
    // <=>
    // ( list : filter ft : map n ) * 3 ++ [end_elem]
    //
    // list : filter ft : map n^3 ++ [end_elem]
    // <=>
    // list : filter ft : map ( n^3 ) ++ [end_elem]
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
            "x",
            "x + y",
            "x+y",
            "x+y+z",
            "(x+y)*7",
            "(x+y) : double",
            "n^8",
            "√2",
            "-√2",
            "n/8+9:something",
            "list : filter ft : map n * 3 ++ [end_elem]",
            "cos x*2",
            "(cos x)*2",
        ];

        for i in input.iter() {
            let (rest, _fe) = FullExpression::parse(i).unwrap();
            assert_eq!(rest, "");
        }
    }

    #[test]
    fn pipe_test() {
        let input = "[1, 2, 3] : map f : add √4";
        let (rest, _expr) = FullExpression::parse(input).unwrap();
        assert_eq!(rest, "");
    }
}

trait ParseExpression<'a>
where
    Self: Sized,
{
    fn parse(input: &'a str) -> Res<'a, FullExpression>;

    fn parse_ws(input: &'a str) -> Res<'a, FullExpression> {
        ws(Self::parse)(input)
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

        impl<'a> ParseExpression<'a> for $name<'a> {
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
        }
    };
}

create_ast_expr!(And, keywords::And, Or);
create_ast_expr!(Or, keywords::Or, Concat);
create_ast_expr!(Concat, keywords::Concat, Pipe);
create_ast_expr!(Add, keywords::Add, Subtract);
create_ast_expr!(Subtract, keywords::Subtract, Multiply);
create_ast_expr!(Multiply, keywords::Multiply, Divide);
create_ast_expr!(Divide, keywords::Divide, Pipe);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pipe<'a> {
    pub span: &'a str,
    pub expr: Box<Expression<'a>>,
    pub function_call: FunctionCall<'a>,
}

impl<'a> ParseExpression<'a> for Pipe<'a> {
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
