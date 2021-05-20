use crate::ast::{
    expr::{BlockExpression, FullExpression, StringLiteral},
    identifier::{FullIdentifier, Identifier},
    type_signature::TypeSignature,
};

use crate::ast::*;
use crate::parse::*;
use crate::util::*;

pub enum FunctionOrTypeOrTest<'a> {
    Function(Function<'a>),
    TypeDecl(TypeDecl<'a>),
    Test(Test<'a>),
}

pub struct Function<'a> {
    pub span: &'a str,
    pub generic_stub: Option<GenericStub<'a>>,
    pub public: bool,
    pub name: Identifier<'a>,
    pub parameters: Vec<(Identifier<'a>, TypeSignature<'a>)>,
    pub return_type: TypeSignature<'a>,
    pub instructions: FullExpression<'a>,
}

// generic A, B where C = add(A, B)
pub struct GenericStub<'a> {
    pub span: &'a str,
    pub generic_arguments: Vec<Identifier<'a>>,
    pub where_clauses: Vec<WhereClause<'a>>,
}

impl<'a> Parse<'a> for GenericStub<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        use nom::multi::{many0, separated_list1};
        let (rest, _) = keywords::Generic::parse(input)?;
        // TODO no recover from here
        // A, B, C
        let (rest, generic_arguments) = separated_list1(keywords::Comma::parse_ws, Identifier::parse_ws)(rest)?;
        // where
        let (rest, _) = keywords::Where::parse_ws(rest)?;
        let (rest, where_clauses) = many0(WhereClause::parse_ws)(rest)?;

        let span = unsafe {from_to(input, rest) };

        Ok((rest, GenericStub{span, generic_arguments, where_clauses}))
    }
}


// C = mul(A, B)
pub struct WhereClause<'a> {
    pub span: &'a str,
    pub generic_destination: Identifier<'a>,
    pub function: FullIdentifier<'a>,
    pub generic_function_arguments: Vec<Identifier<'a>>,
}

impl<'a> Parse<'a> for WhereClause<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        use nom::multi::separated_list0;
        let (rest, generic_destination) = Identifier::parse(input)?;
        // TODO no recover after

        // =
        let (rest, _) = keywords::Assign::parse_ws(rest)?;

        let (rest, function) = FullIdentifier::parse_ws(rest)?;

        let (rest, generic_function_arguments) = separated_list0(keywords::Comma::parse_ws, Identifier::parse_ws)(rest)?;

        let span = unsafe { from_to(input, rest) };
        Ok((rest, WhereClause {span, generic_destination, function, generic_function_arguments}))
    }
}

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

pub struct TypeDecl<'a> {
    pub span: &'a str,
    pub name: Identifier<'a>,
    pub generic_args_decl: Option<GenericArgsDecl<'a>>,
    pub fields: EnumOrStructFields<'a>,
}

impl<'a> Parse<'a> for TypeDecl<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        use nom::combinator::opt;

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

pub struct GenericArgsDecl<'a> {
    pub span: &'a str,
    pub generic_arguments: Vec<Identifier<'a>>,
}

impl<'a> Parse<'a> for GenericArgsDecl<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        use nom::multi::many0;
        let (rest, generic_arguments) = many0(Identifier::parse_ws)(input)?;

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

pub enum EnumOrStructFields<'a> {
    EnumFields(Vec<EnumField<'a>>),
    StructFields(Vec<StructField<'a>>),
}

impl<'a> Parse<'a> for EnumOrStructFields<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        use nom::{branch::alt, combinator::map, multi::many1};
        alt((
            map(many1(EnumField::parse_ws), EnumOrStructFields::EnumFields),
            map(
                many1(StructField::parse_ws),
                EnumOrStructFields::StructFields,
            ),
        ))(input)
    }
}

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
        let (rest, value_type) = nom::combinator::opt(TypeSignature::parse_ws)(rest)?;

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
        use nom::{branch::alt, combinator::map};

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
