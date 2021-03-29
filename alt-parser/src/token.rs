
/// Wrapper for every syntactic element in the ast
/// holds information about preceding whitespace or comments,
/// as well as position of the Token in source code.
pub struct Token<'a, Data, Doc> {
    pub document: Doc,
    pub preceding: Vec<NoInfo<'a>>,
    pub span: &'a str,
    pub from: usize,
    pub to: usize,
    pub value: Data,
}

/// Holds information about semantically irrelevant information,
/// Whitespaces and Comments
pub enum NoInfo<'a> {
    Whitespaces(&'a str),
    Newline(&'a str),
    LineComment(&'a str),
}
