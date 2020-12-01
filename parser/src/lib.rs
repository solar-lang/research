mod expression;
mod generics;
mod identifier;
mod keyword;
mod parse;
mod structs;
mod types;
mod util;
use nom_locate::LocatedSpan;

pub(crate) use parse::Parse;

pub type Span<'a> = LocatedSpan<&'a str>;
