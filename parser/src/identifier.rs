use crate::util::characters::*;
use crate::*;


#[derive(Clone, Debug)]
pub struct Identifier<'a> {
    pub name: &'a str,
}

impl<'a> Parse<'a> for Identifier<'a> {
    fn parse(s: Span<'a>) -> nom::IResult<Span, (Span, Self)> {
        use nom::bytes::complete::{take_while, take_while1};
        use nom::combinator::recognize;
        use nom::sequence::pair;

        let (rest, span) = recognize(pair(take_while1(alpha), take_while(identchar)))(s)?;

        // TODO identifier validation

        // if !is_valid(&span) {
        //     return Err(nom::Err::Error((s, nom::error::ErrorKind::Tag)));
        // }

        // if !keyword::is_keyword(&span) {
        //     return Err(nom::Err::Error((s, nom::error::ErrorKind::Tag)));
        // }

        let identifier = Identifier {
            name: &span,
        };

        Ok((rest, (span, identifier)))
    }
}

// check last char of a string is not an underscore
fn is_valid(s: &str) -> bool {
    let slice = s.as_bytes();
    if let Some(last_char) = slice.iter().nth_back(0) {
        return *last_char != b'_';
    }

    // empty string shouldn't happen. But it's wrong if it does
    false
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_ident_validation() {
        assert_eq!(is_valid("hello_"), false);
        assert_eq!(is_valid("hello_world"), true);
    }

    #[test]
    fn identifier_parsing() {
        let (rest, ident) = Identifier::parse(Span::from("Something_beaut1fu7 ")).unwrap();
        assert_eq!(*rest, " ");
        assert_eq!(ident.name, "Something_beaut1fu7");
    }
}
