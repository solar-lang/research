use crate::ast::{type_signature::TypeSignature, identifier::{FullIdentifier, Identifier }, expr::{StringLiteral, FullExpression as Expression } };

use crate::parse::*;
use crate::ast::*;
use crate::util::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UseImport<'a> {
    pub span: &'a str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Import<'a> {
    pub span: &'a str,
    pub from: Identifier<'a>,
    pub select: Option<Box<ImportSelector<'a>>>,
}

impl<'a> Parse<'a> for Import<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, from) = Identifier::parse(input)?;

        match ImportSelector::parse_ws(rest) {
            Ok((rest, select)) => {
                let select = Some(Box::new(select));
                let span = unsafe { from_to(input, rest) };

                Ok((rest, Import {span, from, select}))
            },
            _ => {
                let span = unsafe { from_to(input, rest) };
                Ok((rest, Import { span, from, select: None}))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imports() {
        let input = "std.collections(hashmap vector util..)  ";
        let imports = Import::parse(input);
        assert!(imports.is_ok());
        let (rest, imports) = imports.unwrap();
        assert_eq!(rest, "  ");
        assert_eq!(imports.span, &input[..( input.len() -2 )]);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ImportSelector<'a> {
    // .. (spread)
    Everything { span: &'a str },
    // .xyz
    Package(Import<'a>),
    Packages(Vec<Import<'a>>),
}

impl<'a> Parse<'a> for ImportSelector<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        use nom::{sequence::{delimited, preceded }, combinator::map, branch::alt, multi::many1 };
        use keywords::*;

        alt((
            map(KeywordSpread::parse, |KeywordSpread {span}| ImportSelector::Everything {span}),
            map(preceded(KeywordDot::parse_ws, Import::parse_ws ), ImportSelector::Package),
            map(delimited(KeywordParenOpen::parse, many1(Import::parse_ws), KeywordParenClose::parse_ws), ImportSelector::Packages),
                ))(input)
    }
}


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
    pub instructions: Expression<'a>,
}

pub struct GenericStub<'a> {
    pub span: &'a str,
    pub generic_arguments: Vec<Identifier<'a>>,
    pub where_clauses: Vec<WhereClause<'a>>,
}

// TODO: the where clause might change later.
// Currently this is possible:
// C = mul(A, B)
//
// this is not:
// somef(List A, fn A -> B) -> List N
pub struct WhereClause<'a> {
    pub span: &'a str,
    pub generic_destination: Identifier<'a>,
    pub function: FullIdentifier<'a>,
    pub generic_function_arguments: Vec<Identifier<'a>>,
}

pub struct Test<'a> {
    pub span: &'a str,
    pub name: StringLiteral<'a>,
    pub instructions: Expression<'a>,
}

pub struct TypeDecl<'a> {
    pub span: &'a str,
    pub name: Identifier<'a>,
    pub generic_args_decl: Option<GenericArgsDecl<'a>>,
    pub fields: EnumOrStructFields<'a>,
}

pub struct GenericArgsDecl<'a> {
    pub span: &'a str,
    pub generic_arguments: Vec<Identifier<'a>>,
}

pub enum EnumOrStructFields<'a> {
    EnumFields(Vec<EnumField<'a>>),
    StructFields(Vec<StructField<'a>>),
}

pub struct EnumField<'a> {
    pub span: &'a str,
    pub name: Identifier<'a>,
    pub value_type: Option<TypeSignature<'a>>,
}

pub struct StructField<'a> {
    pub span: &'a str,
    pub public: bool,
    pub mutable: bool,
    pub name: Identifier<'a>,
    pub value_type: TypeSignature<'a>,
}
