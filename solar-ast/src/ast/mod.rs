pub mod keywords;
pub mod type_signature;
pub mod identifier;
pub mod body;
pub mod expr;
use body::{Import, FunctionOrTypeOrTest};

/// Tree representation of the syntax of a solar file
pub struct Ast<'a> {
    pub span: &'a str,
    pub imports: Vec<Import<'a>>,
    pub functions_and_types_and_tests: Vec<FunctionOrTypeOrTest<'a>>,
}

