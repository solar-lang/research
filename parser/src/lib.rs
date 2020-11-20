mod generics;
mod identifier;
mod keyword;
mod parse;
mod structs;
mod util;

pub(crate) use parse::Parse;

pub(crate) use nom_locate::{position, LocatedSpan};

pub type Span<'a> = LocatedSpan<&'a str>;
