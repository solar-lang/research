use crate::parse::{Parse, Res};
use nom::{bytes::complete::tag, combinator::map};

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

keyword!(Mut, "mut");
keyword!(Pipe, ":");
keyword!(Type, "type");
keyword!(Abs, "|");
keyword!(Dot, ".");
keyword!(Spread, "..");
keyword!(ThinArrow, "->");
keyword!(FatArrow, "=>");
keyword!(Comma, ",");
keyword!(Plus, "+");
keyword!(Minus, "-");
keyword!(Test, "test");
keyword!(Assign, "=");
keyword!(Generic, "generic");
keyword!(Where, "where");
keyword!(When, "when");
keyword!(Public, "pub");
keyword!(And, "and");
keyword!(Or, "or");
keyword!(ParenClose, ")");
keyword!(ParenOpen, "(");
keyword!(BracketClose, "]");
keyword!(BracketOpen, "[");
keyword!(CurlyClose, "}");
keyword!(CurlyOpen, "{");
keyword!(Smaller, "<");
keyword!(Greater, ">");
keyword!(SmallerEqual, "<=");
keyword!(GreaterEqual, ">=");
keyword!(Equal, "==");
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
keyword!(Is, "is");
keyword!(Then, "then");
keyword!(Comment, "#");
keyword!(StringStart, "\"");
keyword!(CharStart, "'");
