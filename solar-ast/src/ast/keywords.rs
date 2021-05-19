use crate::parse::{Parse, Res};
use nom::bytes::complete::tag;
use nom::combinator::map;

pub struct KeywordDot<'a> {
    pub span: &'a str,
}

impl<'a> Parse<'a> for KeywordDot<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        map(tag("."), |span| KeywordDot { span })(input)
    }
}

pub struct KeywordSpread<'a> {
    pub span: &'a str,
}

impl<'a> Parse<'a> for KeywordSpread<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        map(tag(".."), |span| KeywordSpread { span })(input)
    }
}

pub struct KeywordParenOpen<'a> {
    pub span: &'a str,
}

impl<'a> Parse<'a> for KeywordParenOpen<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        map(tag("("), |span| KeywordParenOpen { span })(input)
    }
}

pub struct KeywordParenClose<'a> {
    pub span: &'a str,
}

impl<'a> Parse<'a> for KeywordParenClose<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        map(tag(")"), |span| KeywordParenClose { span })(input)
    }
}
