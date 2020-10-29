use crate::util::characters::*;
use crate::*;

pub struct Identifier<'a> {
    pub name: String,
    pub pos: Span<'a>,
}

// TODO escape keywords!
impl Parse for Identifier<'_> {
    fn parse(s: Span) -> nom::IResult<Span, Self> {
        use nom::bytes::complete::take_while;
        use nom::combinator::recognize;
        use nom::sequence::pair;

        let (rest, name) = recognize(pair(take_while(alpha), take_while(identchar)))(s)?;

        // TODO validate structure
        // TODO check, if this is not a keyword

        let identifier = Identifier {
            name: format!("{}", name),
            pos: name,
        };

        Ok((rest, identifier))
    }
}
