mod generics;
mod identifier;
mod keyword;
mod parse;
mod structs;
mod types;
mod util;

pub(crate) use parse::{Parse, StrParse};

pub(crate) use nom_locate::LocatedSpan;

pub type Span<'a> = LocatedSpan<&'a str>;
