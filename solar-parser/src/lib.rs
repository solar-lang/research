mod identifiers;
mod parse;
mod util;

use nom_locate::{position, LocatedSpan};

pub type Span<'a> = LocatedSpan<&'a str>;
