use crate::parse::{Parse, Res};
use nom::bytes::complete::tag;
use nom::combinator::map;

pub struct Dot<'a> {
    pub span: &'a str,
}

impl<'a> Parse<'a> for Dot<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        map(tag("."), |span| Dot { span })(input)
    }
}

pub struct Spread<'a> {
    pub span: &'a str,
}

impl<'a> Parse<'a> for Spread<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        map(tag(".."), |span| Spread { span })(input)
    }
}

pub struct ParenOpen<'a> {
    pub span: &'a str,
}

impl<'a> Parse<'a> for ParenOpen<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        map(tag("("), |span| ParenOpen { span })(input)
    }
}

pub struct ParenClose<'a> {
    pub span: &'a str,
}

impl<'a> Parse<'a> for ParenClose<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        map(tag(")"), |span| ParenClose { span })(input)
    }
}



pub struct BracketOpen<'a> {
    pub span: &'a str,
}

impl<'a> Parse<'a> for BracketOpen<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        map(tag("["), |span| BracketOpen { span })(input)
    }
}

pub struct BracketClose<'a> {
    pub span: &'a str,
}

impl<'a> Parse<'a> for BracketClose<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        map(tag("]"), |span| BracketClose { span })(input)
    }
}


pub struct CurlyOpen<'a> {
    pub span: &'a str,
}

impl<'a> Parse<'a> for CurlyOpen<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        map(tag("{"), |span| CurlyOpen { span })(input)
    }
}

pub struct CurlyClose<'a> {
    pub span: &'a str,
}

impl<'a> Parse<'a> for CurlyClose<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        map(tag("}"), |span| CurlyClose { span })(input)
    }
}




pub struct ThinArrow<'a> {
    pub span: &'a str,
}

impl<'a> Parse<'a> for ThinArrow<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        map(tag("->"), |span| ThinArrow { span })(input)
    }
}

pub struct FatArrow<'a> {
    pub span: &'a str,
}

impl<'a> Parse<'a> for FatArrow<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        map(tag("=>"), |span| FatArrow { span })(input)
    }
}

pub struct Comma<'a> {
    pub span: &'a str,
}

impl<'a> Parse<'a> for Comma<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        map(tag(","), |span| Comma { span })(input)
    }
}
