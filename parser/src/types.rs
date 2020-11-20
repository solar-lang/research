use crate::{Parse, Span};

/// Represents a type with full generics attached.
/// e.g.
/// let x: Array String
/// let m: Map (String, Int64)
pub struct Type<'a> {
    // Name of type
    pub name: String,

    // Generic parameters
    pub params: Vec<Type<'a>>,
    pub pos: Span<'a>,
}
