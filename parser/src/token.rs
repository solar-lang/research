use crate::{Span, unused::Unused };

/// Wrapper holding part of the AST, includes information aboput predceding comments and the location of the Syntax Node.
/// This way the complete Source Code should be recreatable from the AST
pub struct Token<'a, T> {
    pub preceding: Vec<Unused<'a>>,
    pub span: Span<'a>,
    pub content: T,
}
