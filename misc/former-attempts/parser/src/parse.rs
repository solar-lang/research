use crate::{Span, Token, Unused};

pub trait Parse<'a>
where
    Self: Sized + Clone + std::fmt::Debug,
{
    // TODO the second span can be ommited, it will always be {length: startIndex(output.rest) - startIndex(input), index: startIndex(input)}
    fn parse_direct(s: Span<'a>) -> nom::IResult<Span<'a>, Self>;

    fn parse(input: Span<'a>) -> nom::IResult<Span<'a>, (Span<'a>, Self)> {
        let (rest, content): (Span, Self) = Self::parse_direct(input)?;

        let fragment = crate::util::slice_before_offset(input.fragment(), rest.fragment());
        let span = unsafe {Span::new_from_raw_offset(input.location_offset(), input.location_line(), fragment, ()) };
        
        Ok((rest, (span, content)))
    }
    
    fn parse_ws(s: Span<'a>) -> nom::IResult<Span<'a>, Token<'a, Self>> {
        let (rest, preceding) = Unused::parse_many0(s).unwrap();
        let (rest, (span, content)) = Self::parse(rest)?;
        Ok((rest, Token {preceding, span, content}))
    }
}
