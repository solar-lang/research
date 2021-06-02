use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while, take_while1, take_while_m_n},
    combinator::map,
    multi::many1,
    number::complete::recognize_float,
    sequence::preceded,
};

use super::Expression;

use crate::{
    util::characters::{digit10, digit16, digit2, digit8},
    Parse, Span, Token,
};

#[derive(Clone, Debug)]
pub struct StringType<'a> {
    pub span: Span<'a>,
    pub builder: Vec<StringBuilder<'a>>,
}

impl<'a> Parse<'a> for StringType<'a> {
    fn parse_direct(s: Span<'a>) -> nom::IResult<Span<'a>, Self> {
        let (rest, start) = tag("\"")(s)?;
        let span = start;

        let (rest, builder) = many1(|s| parse_string_content(StringBuilder::default(), s))(rest)?;

        return Ok((rest, StringType { span, builder }));
    }
}

/// Type able to hold information about string interpolation stuff
#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct StringBuilder<'a> {
    pub content: String,
    pub expression: Option<Token<'a, Expression<'a>>>,
}

// TODO delete this. I'm too ashamed of this mess
// I've written a lot of bad code, this might be the worst yet
fn parse_string_content<'a>(
    mut builder: StringBuilder<'a>,
    s: Span<'a>,
) -> nom::IResult<Span<'a>, StringBuilder<'a>> {
    let (rest, content) = take_while(|c| c != '"' && c != '\\')(s)?;

    if rest.len() == 0 {
        // TODO error here
    }

    // check if character was a " (end of string)
    if rest.as_bytes()[0] == b'"' {
        // End of string is reached
        return Ok((
            rest,
            StringBuilder {
                content: content.to_string(),
                expression: None,
            },
        ));
    }

    // push content onto string
    builder.content.push_str(*content);

    // we now know, that an \ character caused us to stop parsing
    let (rest, _) = take(1usize)(rest)?;

    match take(1usize)(rest)? {
        (rest, span) if *span == "n" => {
            builder.content.push('\n');
            parse_string_content(builder, rest)
        }
        (rest, span) if *span == "r" => {
            builder.content.push('\r');
            parse_string_content(builder, rest)
        }
        (rest, span) if *span == "t" => {
            builder.content.push('\t');
            parse_string_content(builder, rest)
        }
        (rest, span) if *span == "u" => {
            // ascii characters it is!
            let (rest, code) = take_while_m_n(2, 2, digit16)(rest)?;
            // this is ugly, ugly code
            let unichar = u64::from_str_radix(*code, 16).unwrap() as u8 as char;
            builder.content.push(unichar);
            parse_string_content(builder, rest)
        }
        (rest, span) if *span == "\"" || *span == "\\" => {
            builder.content.push_str(*span);
            parse_string_content(builder, rest)
        }
        (rest, span) if *span == "(" => {
            // String Interpolation! We now want to parse an expression
            let (rest, expression) = Expression::parse_ws(rest)?;
            let (rest, _) = tag(")")(rest)?;
            Ok((
                rest,
                StringBuilder {
                    expression: Some(expression),
                    ..builder
                },
            ))
        }
        (rest, span) => {
            // technically we'd like to error here
            builder.content.push_str(*span);
            parse_string_content(builder, rest)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parsing_str_parts() {
        let input = Span::new("hello world\"");
        let (rest, builder) = parse_string_content(StringBuilder::default(), input).unwrap();

        assert_eq!(*rest, "\"");
        assert_eq!(
            builder,
            StringBuilder {
                content: String::from("hello world"),
                expression: None,
            }
        );
    }

    #[test]
    fn parsing_str_parts_with_expression() {
        let input = Span::new("hello \\(<expr>) \"");
        let (rest, builder) = parse_string_content(StringBuilder::default(), input).unwrap();

        assert_eq!(*rest, " \"");
        assert_eq!(builder.content, String::from("hello "));
    }

    #[test]
    fn parsing_str_parts_escaped() {
        let input = Span::new("hello \\u33\"");
        let (rest, builder) = parse_string_content(StringBuilder::default(), input).unwrap();

        assert_eq!(*rest, "\"");
        assert_eq!(builder.content, String::from("hello \u{33}"));
    }
}
