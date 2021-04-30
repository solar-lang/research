mod identifier;
mod body;
use body::{Import, FunctionOrTypeOrTest};
// use solar_tokenizer::Token;

/// Tree representation of the syntax of a solar file
pub struct Ast<'a> {
    pub imports: Vec<Import<'a>>,
    pub functions_and_types_and_tests: Vec<FunctionOrTypeOrTest<'a>>,
}

