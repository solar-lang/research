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
    };
}

keyword!(Abs, "|");
keyword!(Add, "+");
keyword!(And, "and");
keyword!(Assign, "=");
keyword!(BracketClose, "]");
keyword!(BracketOpen, "[");
keyword!(Break, "break");
keyword!(Colon, ":");
keyword!(Comma, ",");
keyword!(Comment, "#");
keyword!(Concat, "++");
keyword!(CurlyClose, "}");
keyword!(CurlyOpen, "{");
keyword!(Divide, "/");
keyword!(Do, "do");
keyword!(Dot, ".");
keyword!(Equal, "==");
keyword!(FatArrow, "=>");
keyword!(For, "for");
keyword!(Generic, "generic");
keyword!(Greater, ">");
keyword!(GreaterEqual, ">=");
keyword!(If, "if");
keyword!(In, "in");
keyword!(Is, "is");
keyword!(Let, "let");
keyword!(Loop, "loop");
keyword!(Minus, "-");
keyword!(Multiply, "*");
keyword!(Mut, "mut");
keyword!(Next, "next");
keyword!(Not, "!");
keyword!(Or, "or");
keyword!(ParenClose, ")");
keyword!(ParenOpen, "(");
keyword!(Pipe, ":");
keyword!(Plus, "+");
keyword!(Power, "^");
keyword!(Public, "pub");
keyword!(Return, "return");
keyword!(SemiColon, ";");
keyword!(Smaller, "<");
keyword!(SmallerEqual, "<=");
keyword!(Spread, "..");
keyword!(Sqrt, "√");
keyword!(StringStart1, "\"");
keyword!(StringStart2, "'");
keyword!(Subtract, "-");
keyword!(Test, "test");
keyword!(Then, "then");
keyword!(ThinArrow, "->");
keyword!(Type, "type");
keyword!(When, "when");
keyword!(Else, "else");
keyword!(Where, "where");
keyword!(InlineExpressionStart, "$");
// TODO these must be delimited
keyword!(True, "true");
keyword!(False, "false");
