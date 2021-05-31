use crate::{ast::*, parse::*, util::*};
use expr::FullExpression;
use nom::{
    branch::alt,
    bytes::complete::is_not,
    combinator::{map, value, verify},
    multi::many0,
    sequence::{delimited, preceded},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StringLiteral<'a> {
    pub span: &'a str,
    pub value: String,
}

impl<'a> Parse<'a> for StringLiteral<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, string) = verify(IString::parse, |istr| {
            for part in istr.parts.iter() {
                match part {
                    StringPart::InlineExpression(_) => return false,
                    _ => continue,
                }
            }

            true
        })(input)?;

        let IString { span, parts } = string;

        let mut value = String::new();
        for part in parts {
            match part {
                StringPart::InlineExpression(_) => unreachable!(),
                StringPart::Char(c) => value.push(c),
                StringPart::Literal(s) => value.push_str(s),
            }
        }

        Ok((rest, StringLiteral { span, value }))
    }
}

// stolen from https://github.com/Geal/nom/blob/8e09f0c3029d32421b5b69fb798cef6855d0c8df/examples/string.rs#L36-L64
fn parse_unicode(input: &str) -> nom::IResult<&str, char> {
    use nom::bytes::streaming::take_while_m_n;
    use nom::character::streaming::char;
    use nom::combinator::{map_opt, map_res};

    // `take_while_m_n` parses between `m` and `n` bytes (inclusive) that match
    // a predicate. `parse_hex` here parses between 1 and 6 hexadecimal numerals.
    let parse_hex = take_while_m_n(1, 6, |c: char| c.is_ascii_hexdigit());

    // `preceeded` takes a prefix parser, and if it succeeds, returns the result
    // of the body parser. In this case, it parses u{XXXX}.
    let parse_delimited_hex = preceded(
        char('u'),
        // `delimited` is like `preceded`, but it parses both a prefix and a suffix.
        // It returns the result of the middle parser. In this case, it parses
        // {XXXX}, where XXXX is 1 to 6 hex numerals, and returns XXXX
        delimited(
            keywords::ParenOpen::parse,
            parse_hex,
            keywords::ParenClose::parse,
        ),
    );

    // `map_res` takes the result of a parser and applies a function that returns
    // a Result. In this case we take the hex bytes from parse_hex and attempt to
    // convert them to a u32.
    let parse_u32 = map_res(parse_delimited_hex, move |hex| u32::from_str_radix(hex, 16));

    // map_opt is like map_res, but it takes an Option instead of a Result. If
    // the function returns None, map_opt returns an error. In this case, because
    // not all u32 values are valid unicode code points, we have to fallibly
    // convert to char with from_u32.
    map_opt(parse_u32, |value| std::char::from_u32(value))(input)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InlineExpression<'a> {
    pub span: &'a str,
    pub expr: FullExpression<'a>,
}

impl<'a> Parse<'a> for InlineExpression<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, _) = keywords::InlineExpressionStart::parse(input)?;
        let (rest, _) = keywords::ParenOpen::parse_ws(rest)?;
        let (rest, expr) = FullExpression::parse_ws(rest)?;
        let (rest, _) = keywords::ParenClose::parse_ws(rest)?;

        let span = unsafe { from_to(input, rest) };

        Ok((rest, InlineExpression { span, expr }))
    }
}

// Matches \ followed by an escape code
fn parse_escape_codes(input: &str) -> nom::IResult<&str, char> {
    use nom::character::complete::char;

    let tag_escape = char('\\');

    let codes = (
        value('\n', char('n')),
        value('\r', char('r')),
        value('\t', char('t')),
        value('\0', char('0')),
        value('\\', char('\\')),
        value('"', char('"')),
        value('\'', char('\'')),
        // taken from nom examples.
        parse_unicode,
    );
    preceded(tag_escape, alt(codes))(input)
}

// matches part of a string until a special character occurs
// for strings delimited by ""
fn parse_literal1(i: &str) -> nom::IResult<&str, &str> {
    // parse until ", \ or $ occurs
    let matcher = is_not("\"\\$");

    verify(matcher, |s: &str| !s.is_empty())(i)
}

// matches part of a string until a special character occurs
// for strings delimited by ""
fn parse_literal2(i: &str) -> nom::IResult<&str, &str> {
    // parse until ", \ or $ occurs
    let matcher = is_not("'\\$");

    verify(matcher, |s: &str| !s.is_empty())(i)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StringPart<'a> {
    InlineExpression(InlineExpression<'a>),
    Char(char),
    Literal(&'a str),
}

impl<'a> StringPart<'a> {
    // we don't want to implement Parse, because there is no meaning to
    // this item with preceding whitespace
    fn parse1(input: &'a str) -> Res<'a, StringPart<'a>> {
        alt((
            map(InlineExpression::parse, StringPart::InlineExpression),
            map(parse_escape_codes, StringPart::Char),
            map(parse_literal1, StringPart::Literal),
        ))(input)
    }
    fn parse2(input: &'a str) -> Res<'a, StringPart<'a>> {
        alt((
            map(InlineExpression::parse, StringPart::InlineExpression),
            map(parse_escape_codes, StringPart::Char),
            map(parse_literal2, StringPart::Literal),
        ))(input)
    }
}

/// String with interpolation
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IString<'a> {
    pub span: &'a str,
    pub parts: Vec<StringPart<'a>>,
}

impl<'a> Parse<'a> for IString<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        use keywords::{StringStart1, StringStart2};

        let parse1 = delimited(
            StringStart1::parse,
            many0(StringPart::parse1),
            StringStart1::parse,
        );

        let parse2 = delimited(
            StringStart2::parse,
            many0(StringPart::parse2),
            StringStart2::parse,
        );

        let (rest, parts) = alt((parse1, parse2))(input)?;

        let span = unsafe { from_to(input, rest) };

        Ok((rest, IString { span, parts }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_string_parsing() {
        let inputs = [
            "'this is a string'",
            "\"this is a string as well\"",
            "'this\n, this is an escaped string with non ascii characters öüä \n'",
            "\"this\n, this is an escaped string with non ascii characters öüä \n\"",
            "\"this\n, this is an escaped string \\\" \n\"",
            "'this\n, this is an escaped string \\' with delimiters inside it'",
            "'finally, $('string') interpolation'",
        ];

        for input in inputs.iter() {
            let (rest, _) = IString::parse(input).unwrap();
            assert_eq!(rest, "");
        }
    }
}
