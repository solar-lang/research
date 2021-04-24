use solar_tokenizer::Token;

/// Tree representation of the syntax of a solar file
pub struct Ast<'a> {
    pub imports: Vec<Import<'a>>,
    pub functions_and_types_and_tests: Vec<FunctionOrTypeOrTest<'a>>,
}

pub struct Import<'a>;

pub enum FunctionOrTypeOrTest<'a> {
    Function(Function<'a>),
    TypeDecl(TypeDecl<'a>),
    Test(Test<'a>),
}

pub struct Function<'a> {
    pub tokens: &'a [Token<'a>],
    pub generic_stub: Option<GenericStub<'a>>,
    pub public: bool,
    pub name: Identifier<'a>,
    pub parameters: Vec<(Identifier<'a>, Type<'a>)>,
    pub return_type: Type<'a>,
    pub instructions: Expression<'a>,
}

pub struct GenericStub<'a> {
    pub tokens: &'a [Token<'a>],
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
    pub tokens: &'a [Token<'a>],
    pub generic_destination: Identifier<'a>,
    pub function: FullIdentifier<'a>,
    pub generic_function_arguments: Vec<Identifier<'a>>,
}

pub struct Test<'a> {
    pub tokens: &'a [Token<'a>],
    pub name: StringLiteral<'a>,
    pub instructions: Expression<'a>,
}

pub struct TypeDecl<'a> {
    pub tokens: &'a [Token<'a>],
    pub name: Identifier<'a>,
    pub generic_args_decl: Option<GenericArgsDecl<'a>>,
    pub fields: EnumOrStructFields,
}

pub struct GenericArgsDecl<'a> {
    pub tokens: &'a [Token<'a>],
    pub generic_arguments: Vec<Identifier<'a>>,
}

pub enum EnumOrStructFields<'a> {
    EnumFields(Vec<EnumField<'a>>),
    StructFields(Vec<StructField<'a>>),
}

pub struct EnumField<'a> {
    pub tokens: &'a [Token<'a>],
    pub name: Identifier<'a>,
    pub value_type: Option<Type<'a>>,
}

pub struct StructField<'a> {
    pub tokens: &'a [Token<'a>],
    pub public: bool,
    pub mutable: bool,
    pub name: Identifier<'a>,
    pub value_type: Type<'a>,
}
