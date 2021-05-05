pub mod type_signature;
pub mod identifier;
pub mod body;
use body::{Import, FunctionOrTypeOrTest};
use solar_tokenizer::Token;
// use solar_tokenizer::Token;

/// Tree representation of the syntax of a solar file
pub struct Ast<'a> {
    pub tokens: &'a [Token<'a>],
    pub imports: Vec<Import<'a>>,
    pub functions_and_types_and_tests: Vec<FunctionOrTypeOrTest<'a>>,
}

