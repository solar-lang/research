mod parse;
mod ast;
mod error;
pub use parse::Parse;
pub use error::Error;
pub use error::Res;
pub use ast::Ast;

use solar_tokenizer::Token;

pub fn parse<'a>(tokens: impl Iterator<Item=Token<'a>>) -> Res<'a, Ast<'a>> {
    use solar_tokenizer::only_relevant;

    let tokens: Vec<Token> = only_relevant(tokens).collect();

    Ast::parse(&tokens)
}


