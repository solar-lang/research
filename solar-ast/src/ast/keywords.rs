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


pub struct Plus<'a> {
    pub span: &'a str,
}

impl<'a> Parse<'a> for Plus<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        map(tag("+"), |span| Plus { span })(input)
    }
}


pub struct Minus<'a> {
    pub span: &'a str,
}

impl<'a> Parse<'a> for Minus<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        map(tag("-"), |span| Minus { span })(input)
    }
}

pub struct Mut<'a> {
    pub span: &'a str,
}

impl<'a> Parse<'a> for Mut<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        map(tag("mut"), |span| Mut { span })(input)
    }
}

pub struct Pipe<'a> {
    pub span: &'a str,
}

impl<'a> Parse<'a> for Pipe<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        map(tag(":"), |span| Pipe { span })(input)
    }
}

pub struct Abs<'a> {
    pub span: &'a str,
}

impl<'a> Parse<'a> for Abs<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        map(tag("|"), |span| Abs { span })(input)
    }
}

pub struct Type<'a> {
    pub span: &'a str,
}

impl<'a> Parse<'a> for Type<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        map(tag("type"), |span| Type { span })(input)
    }
}

pub struct Test<'a> {
    pub span: &'a str,
}

impl<'a> Parse<'a> for Test<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        map(tag("test"), |span| Test { span })(input)
    }
}

pub struct Assign<'a> {
    pub span: &'a str,
}

impl<'a> Parse<'a> for Assign<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        map(tag("="), |span| Assign { span })(input)
    }
}


pub struct Generic<'a> {
    pub span: &'a str,
}

impl<'a> Parse<'a> for Generic<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        map(tag("generic"), |span| Generic { span })(input)
    }
}

pub struct Where<'a> {
    pub span: &'a str,
}

impl<'a> Parse<'a> for Where<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        map(tag("where"), |span| Where { span })(input)
    }
}

pub struct When<'a> {
    pub span: &'a str,
}

impl<'a> Parse<'a> for When<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        map(tag("when"), |span| When { span })(input)
    }
}


pub struct Public<'a> {
    pub span: &'a str,
}

impl<'a> Parse<'a> for Public<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        map(tag("pub"), |span| Public { span })(input)
    }
}

pub struct And<'a> {
    pub span: &'a str,
}

impl<'a> Parse<'a> for And<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        map(tag("and"), |span| And { span })(input)
    }
}

macro_rules! keyword {
    ($name:ident, $tag:tt) => {

        pub struct $name<'a> {
            pub span: &'a str,
        }

        impl<'a> Parse<'a> for $name<'a> {
            fn parse(input: &'a str) -> Res<'a, Self> {
                map(tag($tag), |span| $name { span })(input)
            }
        }

    }
}

keyword!(Or, "or");
keyword!(Concat, "++");
keyword!(Add, "+");
keyword!(Subtract, "-");
keyword!(Multiply, "*");
keyword!(Divide, "/");
keyword!(Power, "^");
keyword!(Sqrt, "√");
keyword!(Not, "!");
keyword!(Colon, ":");
keyword!(SemiColon, ";");
keyword!(If, "if");
keyword!(Do, "do");
keyword!(For, "for");
keyword!(In, "in");
keyword!(Loop, "loop");
keyword!(Let, "let");
keyword!(Break, "break");
keyword!(Next, "next");
keyword!(Return, "return");
