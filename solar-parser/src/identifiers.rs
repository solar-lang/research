pub use crate::Span;

pub struct Identifier<'a> {
    pub name: String,
    pub pos: Span<'a>,
}
