mod ast;
mod error;

pub use error::{Error, TokenError};
pub use ast::Ast;
mod parse;
pub use parse::{Tokens, Parse, Res};


use solar_tokenizer::Token;

pub fn parse<'a>(tokens: impl Iterator<Item=Token<'a>>) -> nom::IResult<Tokens<'a>, Ast> {
    use solar_tokenizer::only_relevant;

    let tokens: Vec<Token> = only_relevant(tokens).collect();

    Ast::parse(&tokens)
}


