use crate::ast::expr::when::When;
use crate::ast::identifier::{Identifier, FullIdentifier };
use solar_tokenizer::Token;

/// Expressions
/// The main element of the solar language

type BFE<'a> = Box<FullExpression<'a>>;
pub enum FullExpression<'a> {
    And(BFE<'a>, BFE<'a>),
    Or(BFE<'a>, BFE<'a>),
    Concat(BFE<'a>, BFE<'a>),
    Add(BFE<'a>, BFE<'a>),
    Subtract(BFE<'a>, BFE<'a>),
    Multiply(BFE<'a>, BFE<'a>),
    Divide(BFE<'a>, BFE<'a>),
    Power(BFE<'a>, BFE<'a>),
    Sqrt(BFE<'a>),
    Negate(BFE<'a>),
    Not(BFE<'a>),
    Pipe(BFE<'a>, BFE<'a>),
    // direct field access
    Dot(BFE<'a>, BFE<'a>),
    Expression(Box<Expression<'a>>),
}

//  pub tokens: &'a [Token<'a>],
pub enum Expression<'a> {
    FunctionCall(FunctionCall<'a>),
    Value(Value<'a>),
}

pub enum Value<'a> {
    Litaral(Literal<'a>),
    FullIdentifier(FullIdentifier<'a>),
    Closure(Closure<'a>),
    Array(Array<'a>),
    Abs(BFE<'a>),
    Parenthesis(BFE<'a>),
    Tuple(Vec<FullExpression<'a>>),
    When(When<'a>),
    BlockExpression(BlockExpression<'a>),
}

pub struct Array<'a> {
    pub tokens: &'a[Token<'a>],
    pub values: Vec<FullExpression<'a>>,
}

pub struct FunctionCall<'a> {
    pub tokens: &'a[Token<'a>],
    // Note: may as well be a variable
    pub function_name: FullIdentifier<'a>,
    pub args: Vec<FunctionArg<'a>>,
}

pub struct FunctionArg<'a> {
    pub name: Identifier<'a>,
    pub value: Value<'a>
}

pub enum Literal<'a> {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(StringLiteral<'a>),
}

pub struct StringLiteral<'a> {
    tokens: &'a [Token<'a>],
    value: String
}


// NOTE: Quite complicated, expect for iterative changes
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
        pub tokens: &'a [Token<'a>],
        pub subguards: Vec<Guard<'a>>,
        pub rest: Option<Identifier<'a>>,
    }

    pub struct ObjectGuard<'a> {
        pub tokens: &'a [Token<'a>],
        pub struct_identifier: FullIdentifier<'a>,
        pub fields: Vec<(Identifier<'a>, ParenGuard<'a>)>,
    }

    pub struct TupleGuard<'a> {
        pub tokens: &'a [Token<'a>],
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
        tokens: &'a [Token<'a>],
        branches: Vec<Branch<'a>>,
        else_clause: Option<FullExpression<'a>>,
    }

    pub struct Branch<'a> {
        is: Guard<'a>,
        then: FullExpression<'a>,
    }
}

pub struct Closure<'a> {
    pub tokens: &'a [Token<'a>],
    pub arguments: ClosureArgs<'a>,
    pub body: Box<Expression<'a>>,
}

pub enum ClosureArgs<'a> {
    Single(Identifier<'a>),
    Multiple(Identifier<'a>),
}

pub struct BlockExpression<'a> {
    pub tokens: &'a [Token<'a>],
    pub parts: Vec<BlockExpressionPart<'a>>,
}

pub enum BlockExpressionPart<'a> {
    Let {identifier: Identifier<'a>, expr: FullExpression<'a>},
    Return(Option<FullExpression<'a>>),
    Break,
    Next,
    Expression(FullExpression<'a>),
    Lopp(BlockExpression<'a>),
    If {condition: FullExpression<'a>, then: BlockExpression<'a>},
    For{
        variable: Identifier<'a>,
        over: FullExpression<'a>,
        body: BlockExpression<'a>,
    }
}
