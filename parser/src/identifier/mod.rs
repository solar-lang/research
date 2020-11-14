mod keyword;
use crate::util::characters::*;
use crate::*;

pub struct Identifier<'a> {
    pub name: String,
    pub pos: Span<'a>,
}

// TODO escape keywords!
impl<'a> Parse<'a> for Identifier<'a> {
    fn parse(s: Span<'a>) -> nom::IResult<Span, Self> {
        use nom::bytes::complete::take_while;
        use nom::combinator::recognize;
        use nom::sequence::pair;

        let (rest, name) = recognize(pair(take_while(alpha), take_while(identchar)))(s)?;
        if !is_valid(&name) {
            return Err(nom::Err::Error((s, nom::error::ErrorKind::Tag)));
        }

        if !keyword::is_keyword(&name) {
            return Err(nom::Err::Error((s, nom::error::ErrorKind::Tag)));
        }

        // TODO check, if this is not a keyword

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
    let last_char = slice[slice.len() - 1];
    last_char != b'_'
}
