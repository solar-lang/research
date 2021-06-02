mod expression;
mod generics;
mod identifier;
mod keyword;
mod parse;
mod structs;
mod token;
mod types;
mod unused;
mod util;
use nom_locate::LocatedSpan;

pub(crate) use parse::Parse;
pub use token::Token;
pub use unused::Unused;

pub type Span<'a> = LocatedSpan<&'a str>;
