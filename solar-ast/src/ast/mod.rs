pub mod type_signature;
pub mod identifier;
pub mod body;
pub mod expr;
use body::{Import, FunctionOrTypeOrTest};
use solar_tokenizer::Token;
use crate::error::{Res, Error};

/// Tree representation of the syntax of a solar file
pub struct Ast<'a> {
    pub tokens: &'a [Token<'a>],
    pub imports: Vec<Import<'a>>,
    pub functions_and_types_and_tests: Vec<FunctionOrTypeOrTest<'a>>,
}

impl<'a> Ast<'a> {
    pub fn parse(tokens: &'a[Token<'a>]) -> Res<'a, Self> {

    }

}

