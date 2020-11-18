use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token {

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
    AsyncKeyword,
    #[token("async")]
    UseKeyword,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    // #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}
