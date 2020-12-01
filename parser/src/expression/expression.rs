use nom::bytes::complete::tag;
use nom::combinator::map;

use crate::{Parse, Span};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expression<'a> {
    span: Span<'a>,
}

impl<'a> Parse<'a> for Expression<'a> {
    fn parse(s: Span<'a>) -> nom::IResult<Span<'a>, Self> {
        // TODO implement proper Expression parsing
        map(tag("<expr>"), |span| Expression { span })(s)
    }
}
