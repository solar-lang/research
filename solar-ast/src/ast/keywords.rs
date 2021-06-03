use crate::parse::{Parse, Res};
use nom::{
    bytes::complete::tag,
    combinator::{map, not},
    sequence::terminated,
};

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

    ($name:ident, $tag:tt, $not_followed_by:expr) => {
        pub struct $name<'a> {
            pub span: &'a str,
        }

        impl<'a> Parse<'a> for $name<'a> {
            fn parse(input: &'a str) -> Res<'a, Self> {
                let mapping = map(tag($tag), |span| $name { span });
                let condition = not($not_followed_by);

                terminated(mapping, condition)(input)
            }
        }
    };
}

/// takes a single character, which would be a valid identifier part.
/// Useful for recognizing if a keyword continues after a tagged part.
///  
/// e.g. tag("true") might recognize "true_value", even though it's the keyword "true" we're searching for.
fn ident_char(input: &str) -> nom::IResult<&str, char> {
    nom::character::complete::satisfy(|c| {
        c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c >= '0' && c <= '9' || c == '_'
    })(input)
}

keyword!(Abs, "|");
keyword!(Add, "+", tag("+"));
keyword!(And, "and", ident_char);
keyword!(Assign, "=");
keyword!(BracketClose, "]");
keyword!(BracketOpen, "[");
keyword!(Break, "break", ident_char);
keyword!(Colon, ":");
keyword!(Comma, ",");
keyword!(Comment, "#");
keyword!(Concat, "++");
keyword!(CurlyClose, "}");
keyword!(CurlyOpen, "{");
keyword!(Divide, "/");
keyword!(Do, "do", ident_char);
keyword!(Dot, ".");
keyword!(Else, "else", ident_char);
keyword!(Equal, "==");
keyword!(False, "false", ident_char);
keyword!(FatArrow, "=>");
keyword!(For, "for", ident_char);
keyword!(Generic, "generic", ident_char);
keyword!(Greater, ">");
keyword!(GreaterEqual, ">=");
keyword!(If, "if", ident_char);
keyword!(In, "in", ident_char);
keyword!(InlineExpressionStart, "$");
keyword!(Is, "is", ident_char);
keyword!(Let, "let", ident_char);
keyword!(Loop, "loop", ident_char);
keyword!(Minus, "-");
keyword!(Multiply, "*");
keyword!(Mut, "mut", ident_char);
keyword!(Next, "next", ident_char);
keyword!(Not, "!");
keyword!(Or, "or");
keyword!(ParenClose, ")");
keyword!(ParenOpen, "(");
keyword!(Pipe, ":");
keyword!(Plus, "+");
keyword!(Power, "^");
keyword!(Public, "pub", ident_char);
keyword!(Return, "return", ident_char);
keyword!(SemiColon, ";");
keyword!(Smaller, "<");
keyword!(SmallerEqual, "<=");
keyword!(Spread, "..");
keyword!(Sqrt, "√");
keyword!(StringStart1, "\"");
keyword!(StringStart2, "'");
keyword!(Subtract, "-");
keyword!(Test, "test", ident_char);
keyword!(Then, "then", ident_char);
keyword!(ThinArrow, "->");
keyword!(True, "true", ident_char);
keyword!(Type, "type", ident_char);
keyword!(Use, "use", ident_char);
keyword!(When, "when", ident_char);
keyword!(Where, "where", ident_char);
