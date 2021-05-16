use nom::IResult;
use solar_tokenizer::Token;
use crate::Error;

pub type Tokens<'a> = &'a [Token<'a>];
pub type Res<'a, T> = IResult<Tokens<'a>, T, Error<'a>>;

pub trait Parse<'a>
where
    Self: Sized,
{
    fn parse(tokens: Tokens<'a>) -> Res<'a, Self>;
}
