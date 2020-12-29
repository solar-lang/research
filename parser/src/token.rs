use std::ops::Deref;
use std::fmt::Debug;
use crate::{unused::Unused, Span};

/// Wrapper holding part of the AST, includes information about predceding comments and the location of the Syntax Node.
/// This way the complete Source Code should be (mostly) recreatable from the AST
#[derive(Clone, Debug)]
pub struct Token<'a, T> where T: Clone + Debug {
    pub preceding: Vec<Unused<'a>>,
    pub span: Span<'a>,
    pub content: T,
}

impl <'a, T> Token<'a, T> where T: Clone + Debug {
    pub fn located(span: Span<'a>, content: T) -> Self {
        Token {preceding: Vec::new(), span, content}
    }
}

impl<'a, T> Deref for Token<'a, T> where T: Clone + Debug {
    type Target = T;

    fn deref(&self) -> &T {
        &self.content
    }
}