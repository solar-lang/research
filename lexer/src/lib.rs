use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[token("(")]
    ParenOpen,
    #[token(")")]
    ParenClose,
    #[token("{")]
    CurlyOpen,
    #[token("}")]
    CurlyClose,
    #[token("[")]
    BracketOpen,
    #[token("]")]
    BracketClose,

    #[token(":")]
    DotDot,

    #[token("|")]
    Pipe,

    #[token(".")]
    Dot,

    #[token(",")]
    Comma,

    #[token("->")]
    Arrow,

    // TODO arithmetic op
    //
    // TODO comments

    // Keywords
    #[token("let")]
    LetKeyword,
    #[token("set")]
    SetKeyword,
    #[token("get")]
    GetKeywor,
    #[token("for")]
    ForKeyword,
    #[token("while")]
    WhileKeyword,
    #[token("if")]
    IfKeyword,
    #[token("else")]
    ElseKeyword,
    #[token("then")]
    ThenKeyword,
    #[token("function")]
    FunctionKeyword,
    #[token("return")]
    ReturnKeyword,
    #[token("break")]
    BreakKeyword,
    #[token("match")]
    MatchKeyword,
    #[token("generic")]
    GenericKeyword,
    #[token("type")]
    TypeKeyword,
    #[token("is")]
    IsKeyword,
    #[token("or")]
    OrKeyword,
    #[token("in")]
    InKeyword,
    #[token("const")]
    ConstKeyword,
    #[token("use")]
    UseKeyword,
    #[token("async")]
    AsyncKeyword,
    #[token("pub")]
    PubKeyword,

    // redundancy
    #[regex("( \n\r\t)+")]
    Whitespace,

    // TODO check
    #[regex("[a-zA-Z][a-zA-Z0-9_]*")]
    Identifier,

    // TODO check
    #[regex("(0[xbo])?[0-9A-Fa-f_]*")]
    Integer,

    // TODO check
    #[regex("[0-9]+.[0-9]+(e-?[0-9]+)?")]
    Float,

    // TODO string
    //
    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    // #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

#[cfg(test)]
mod test {
    #[test]
    fn many_tokens() {
        let input = "fn something()"
    }
}
