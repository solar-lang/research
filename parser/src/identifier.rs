use crate::util::characters::*;
use crate::*;

pub struct Identifier<'a> {
    pub name: String,
    pub pos: Span<'a>,
}

impl<'a> Parse<'a> for Identifier<'a> {
    fn parse(s: Span<'a>) -> nom::IResult<Span, Self> {
        use nom::bytes::complete::{take_while, take_while1};
        use nom::combinator::recognize;
        use nom::sequence::pair;

        let (rest, name) = recognize(pair(take_while1(alpha), take_while(identchar)))(s)?;
        if !is_valid(&name) {
            return Err(nom::Err::Error((s, nom::error::ErrorKind::Tag)));
        }

        if !keyword::is_keyword(&name) {
            return Err(nom::Err::Error((s, nom::error::ErrorKind::Tag)));
        }

        let identifier = Identifier {
            name: format!("{}", name),
            pos: name,
        };

        Ok((rest, identifier))
    }
}

// check last char of a string is not an underscore
fn is_valid(s: &str) -> bool {
    let slice = s.as_bytes();
    if let Some(last_char) = slice.iter().nth_back(0) {
        return *last_char != b'_';
    }

    // empty string
    true
}
