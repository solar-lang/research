use crate::{util::whitespace1, Span};
use nom::combinator::map;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take, take_till},
    sequence::delimited,
    multi::many0,
};

/// Syntacitc element that bears no meaning for the code itself
pub enum Unused<'a> {
    Whitespace(Span<'a>),
    Comment(Span<'a>),
}

impl<'a> Unused<'a> {
    pub fn parse_many0(s: Span<'a>) -> IResult<Span<'a>, Vec<Unused<'a>>> {
        many0(Unused::parse)(s)
    }

    pub fn parse(s: Span<'a>) -> IResult<Span<'a>, Unused<'a>> {
        // recognize eol character
        let eol = |c| c == '\n' || c == '\r';
        // parses a comment
        let comment = delimited(tag("#"), take_till(eol), take(1usize));

        let comment = map(comment, Unused::Comment);
        let whitespace = map(whitespace1, Unused::Whitespace);

        alt((comment, whitespace))(s)
    }
}
