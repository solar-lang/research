mod ast;
pub(crate) mod util;

pub mod parse;
pub use ast::Ast;



/*
use solar_tokenizer::Token;
pub fn parse<'a>(span: impl Iterator<Item=Token<'a>>) -> nom::IResult<&'a str, Ast<'a>> {
    use solar_tokenizer::only_relevant;

    let span: Vec<Token> = only_relevant(span).collect();

    //Ast::parse(&span)
    unimplemented!()
}


*/
