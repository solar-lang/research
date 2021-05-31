use crate::ast::{
    expr::{BlockExpression, FullExpression, StringLiteral},
    identifier::{FullIdentifier, Identifier},
    type_signature::TypeSignature,
};

use crate::ast::*;
use crate::parse::*;
use crate::util::*;

use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::{many0, many1, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FunctionOrTypeOrTest<'a> {
    Function(Function<'a>),
    TypeDecl(TypeDecl<'a>),
    Test(Test<'a>),
}

impl<'a> FunctionOrTypeOrTest<'a> {
    pub fn span(&self) -> &str {
        use FunctionOrTypeOrTest::*;
        match self {
            Function(f) => f.span,
            TypeDecl(t) => t.span,
            Test(t) => t.span,
        }
    }
}

impl<'a> Parse<'a> for FunctionOrTypeOrTest<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        alt((
            map(Test::parse, FunctionOrTypeOrTest::Test),
            map(TypeDecl::parse, FunctionOrTypeOrTest::TypeDecl),
            map(Function::parse, FunctionOrTypeOrTest::Function),
        ))(input)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Function<'a> {
    pub span: &'a str,
    pub generic_stub: Option<GenericStub<'a>>,
    pub public: bool,
    pub name: Identifier<'a>,
    pub parameters: Vec<(Identifier<'a>, TypeSignature<'a>)>,
    pub return_type: Option<TypeSignature<'a>>,
    pub instructions: FullExpression<'a>,
}

impl<'a> Parse<'a> for Function<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, generic_stub) = opt(GenericStub::parse)(input)?;
        let (rest, public) = opt(keywords::Public::parse_ws)(rest)?;
        let public = public.is_some();
        // no func keyword. E.g. f(x) = x^2
        let (rest, name) = Identifier::parse_ws(rest)?;

        //  (
        let (rest, _) = keywords::ParenOpen::parse_ws(rest)?;

        // x Int, y Int, fac Float
        let (rest, parameters) = separated_list0(
            keywords::Comma::parse_ws,
            pair(Identifier::parse_ws, TypeSignature::parse_ws),
        )(rest)?;

        //  )
        let (rest, _) = keywords::ParenClose::parse_ws(rest)?;

        let (rest, return_type) = opt(preceded(
            keywords::ThinArrow::parse_ws,
            TypeSignature::parse_ws,
        ))(rest)?;

        let (rest, instructions) =
            preceded(keywords::Assign::parse_ws, FullExpression::parse_ws)(rest)?;

        let span = unsafe { from_to(input, rest) };

        Ok((
            rest,
            Function {
                span,
                generic_stub,
                public,
                name,
                parameters,
                return_type,
                instructions,
            },
        ))
    }
}

// generic A, B where C = add(A, B)
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GenericStub<'a> {
    pub span: &'a str,
    pub generic_arguments: Vec<Identifier<'a>>,
    pub where_clauses: Vec<WhereClause<'a>>,
}

impl<'a> Parse<'a> for GenericStub<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, _) = keywords::Generic::parse(input)?;
        // TODO no recover from here
        // A, B, C
        let (rest, generic_arguments) =
            separated_list1(keywords::Comma::parse_ws, Identifier::parse_ws)(rest)?;
        // where
        let (rest, _) = keywords::Where::parse_ws(rest)?;
        let (rest, where_clauses) = many0(WhereClause::parse_ws)(rest)?;

        let span = unsafe { from_to(input, rest) };

        Ok((
            rest,
            GenericStub {
                span,
                generic_arguments,
                where_clauses,
            },
        ))
    }
}

// C = mul(A, B)
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WhereClause<'a> {
    pub span: &'a str,
    pub generic_destination: Identifier<'a>,
    pub function: FullIdentifier<'a>,
    pub generic_function_arguments: Vec<Identifier<'a>>,
}

impl<'a> Parse<'a> for WhereClause<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, generic_destination) = Identifier::parse(input)?;
        // TODO no recover after

        // =
        let (rest, _) = keywords::Assign::parse_ws(rest)?;

        let (rest, function) = FullIdentifier::parse_ws(rest)?;

        let (rest, generic_function_arguments) =
            separated_list0(keywords::Comma::parse_ws, Identifier::parse_ws)(rest)?;

        let span = unsafe { from_to(input, rest) };
        Ok((
            rest,
            WhereClause {
                span,
                generic_destination,
                function,
                generic_function_arguments,
            },
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Test<'a> {
    pub span: &'a str,
    pub name: StringLiteral<'a>,
    pub instructions: BlockExpression<'a>,
}

impl<'a> Parse<'a> for Test<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, _) = keywords::Test::parse(input)?;

        // (TODO) can't recover from here on
        let (rest, name) = expr::StringLiteral::parse_ws(rest)?;
        let (rest, instructions) = expr::BlockExpression::parse_ws(rest)?;

        let span = unsafe { from_to(input, rest) };

        Ok((
            rest,
            Test {
                span,
                name,
                instructions,
            },
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeDecl<'a> {
    pub span: &'a str,
    pub name: Identifier<'a>,
    pub generic_args_decl: Option<GenericArgsDecl<'a>>,
    pub fields: EnumOrStructFields<'a>,
}

impl<'a> Parse<'a> for TypeDecl<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, _) = keywords::Type::parse(input)?;
        let (rest, name) = Identifier::parse_ws(rest)?;
        let (rest, generic_args_decl) = opt(GenericArgsDecl::parse_ws)(rest)?;
        let (rest, fields) = EnumOrStructFields::parse_ws(rest)?;

        let span = unsafe { from_to(input, rest) };

        Ok((
            rest,
            TypeDecl {
                span,
                name,
                generic_args_decl,
                fields,
            },
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GenericArgsDecl<'a> {
    pub span: &'a str,
    pub generic_arguments: Vec<Identifier<'a>>,
}

impl<'a> Parse<'a> for GenericArgsDecl<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        use keywords::*;
        let (rest, generic_arguments) = alt((
            map(Identifier::parse, |i| vec![i]),
            delimited(
                ParenOpen::parse,
                separated_list1(Comma::parse_ws, Identifier::parse_ws),
                ParenClose::parse_ws,
            ),
        ))(input)?;

        let span = unsafe { from_to(input, rest) };

        Ok((
            rest,
            GenericArgsDecl {
                span,
                generic_arguments,
            },
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EnumOrStructFields<'a> {
    EnumFields(Vec<EnumField<'a>>),
    StructFields(Vec<StructField<'a>>),
}

impl<'a> Parse<'a> for EnumOrStructFields<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        alt((
            map(many1(EnumField::parse_ws), EnumOrStructFields::EnumFields),
            map(
                many1(StructField::parse_ws),
                EnumOrStructFields::StructFields,
            ),
        ))(input)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnumField<'a> {
    pub span: &'a str,
    pub name: Identifier<'a>,
    pub value_type: Option<TypeSignature<'a>>,
}

impl<'a> Parse<'a> for EnumField<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        //      |
        let (rest, _) = keywords::Abs::parse(input)?;
        let (rest, name) = Identifier::parse_ws(rest)?;
        let (rest, value_type) = opt(TypeSignature::parse_ws)(rest)?;

        let span = unsafe { from_to(input, rest) };

        Ok((
            rest,
            EnumField {
                span,
                name,
                value_type,
            },
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructField<'a> {
    pub span: &'a str,
    pub public: bool,
    pub mutable: bool,
    pub name: Identifier<'a>,
    pub value_type: TypeSignature<'a>,
}

impl<'a> Parse<'a> for StructField<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        use keywords::{Minus, Mut, Plus};

        // + or -
        let (rest, public) =
            alt((map(Plus::parse, |_| true), map(Minus::parse, |_| false)))(input)?;

        // mut
        let (rest, mutable) = if let Ok((rest, _)) = Mut::parse_ws(rest) {
            (rest, true)
        } else {
            (rest, false)
        };

        let (rest, name) = Identifier::parse_ws(rest)?;

        let (rest, value_type) = TypeSignature::parse_ws(rest)?;

        let span = unsafe { from_to(input, rest) };

        Ok((
            rest,
            StructField {
                span,
                public,
                mutable,
                name,
                value_type,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn types() {
        let input = [
            "type TrafficLight | Red | Yellow | Green | RedYellow",
            "type Gender | Female | Male | Other String",
            "type Option T | Some T | None",
            "type Result (R, E) | Ok R | Err E",
            "type Point - x Float - y Float",
            "type Point + x Float + y Float",
            "type Point T + x T + y T",
            "type Person + birthday Date + name String + mut gender Gender",
        ];

        for i in &input {
            let (rest, value) = TypeDecl::parse(i).unwrap();
            // here we just test if the entire input was consumed
            assert_eq!(rest, "");
            assert_eq!(&value.span, i);
        }
    }
}
