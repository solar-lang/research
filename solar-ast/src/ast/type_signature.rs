use solar_tokenizer::Token;
use crate::ast::identifier::FullIdentifier;

pub struct TypeSignature<'a> {
    pub span: &'a str,
    pub type_kind: TypeKind<'a>,
    pub return_type: Option<Box<TypeSignature<'a>>>,
}

pub enum TypeKind<'a> {
    DirectType(DirectType<'a>),
    TupleType(TupleType<'a>),
    VectorType(VectorType<'a>),
}

pub struct DirectType<'a> {
    pub span: &'a str,
    // may contain (relative) path to type
    pub full_identifier: FullIdentifier<'a>,
    pub generic_argument: Option<Box<TypeSignature<'a>>>,
}

pub struct TupleType<'a> {
    pub span: &'a str,
    // may contain (relative) path to type
    pub types: Vec<TypeSignature<'a>>,
}

// Just the same as Vec <generic_argument>
pub struct VectorType<'a> {
    pub span: &'a str,
    pub generic_argument: Box<TypeSignature<'a>>,
}
