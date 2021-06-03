// full expression

use crate::ast::expr::{Expression, FunctionCall};
use crate::{ast::*, parse::*, util::*};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FullExpression<'a> {
    Or(Or<'a>),
    And(And<'a>),
    Concat(Concat<'a>),

    Add(Add<'a>),
    Subtract(Subtract<'a>),

    Multiply(Multiply<'a>),
    Divide(Divide<'a>),

    // Rarely used, because mostly Value::Power takes precedence.
    Power(Power<'a>),

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
        Or::parse(input)
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
                    let (rest, right) = Self::parse_ws(rest)?;
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

create_ast_expr!(Or, keywords::Or, And);
create_ast_expr!(And, keywords::And, Concat);
create_ast_expr!(Concat, keywords::Concat, Add);
create_ast_expr!(Add, keywords::Add, Subtract);
create_ast_expr!(Subtract, keywords::Subtract, Multiply);
create_ast_expr!(Multiply, keywords::Multiply, Divide);
create_ast_expr!(Divide, keywords::Divide, Power);
create_ast_expr!(Power, keywords::Power, Pipe);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pipe<'a> {
    pub span: &'a str,
    pub expr: Box<Expression<'a>>,
    pub function_chain: Vec<FunctionCall<'a>>,
}

impl<'a> ParseExpression<'a> for Pipe<'a> {
    fn parse(input: &'a str) -> Res<'a, FullExpression> {
        use nom::{multi::many1, sequence::preceded};

        let (rest, expr) = Expression::parse(input)?;
        let expr = Box::new(expr);

        let mut parse_function_chain_ws =
            many1(preceded(keywords::Colon::parse_ws, FunctionCall::parse_ws));

        if let Ok((rest, function_chain)) = parse_function_chain_ws(rest) {
            let span = unsafe { from_to(input, rest) };

            return Ok((
                rest,
                Pipe {
                    span,
                    expr,
                    function_chain,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &str) -> FullExpression {
        FullExpression::parse_ws(input).unwrap().1
    }

    // No negation considered in parsing thus far
    #[test]
    fn negation1() {
        let input = "-√2";
        let result = FullExpression::parse(input);
        assert!(result.is_err());
    }

    #[test]
    fn negation2() {
        let input = "a + -b";
        let result = FullExpression::parse(input);
        assert!(result.is_err());
    }

    #[test]
    fn concatination() {
        let input = "a ++ b ";
        let (rest, expr) = FullExpression::parse(input).unwrap();
        assert_eq!(
            expr,
            FullExpression::Concat(Concat {
                span: "a ++ b",
                left: Box::new(parse("a")),
                right: Box::new(parse("b")),
            })
        );

        assert_eq!(rest, " ");
    }

    #[test]
    fn exponent1() {
        let input = "a^b ";
        let (rest, expr) = FullExpression::parse(input).unwrap();
        assert_eq!(
            expr,
            FullExpression::Power(Power {
                span: "a^b",
                left: Box::new(parse("a")),
                right: Box::new(parse("b")),
            })
        );
        assert_eq!(rest, " ");
    }

    #[test]
    fn exponent_right_associative() {
        let input = "a^b^c ";
        let (rest, expr) = FullExpression::parse(input).unwrap();
        assert_eq!(
            expr,
            FullExpression::Power(Power {
                span: "a^b^c",
                left: Box::new(parse("a")),
                right: Box::new(parse("b^c")),
            })
        );
        assert_eq!(rest, " ");
    }

    #[test]
    fn cheap_tests() {
        let input = [
            "x + y^2 + z + 9",
            "x",
            "(x)",
            "x+y",
            "x + y",
            "x+y+z",
            "x+y ++ list",
            "x*y + y",
            "x+y+z+9",
            "x+y+z+9 + 10",
            "(x+y)*7",
            "(x+y) : double",
            "n^8",
            "√2",
            "!true or a",
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
