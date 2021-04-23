use logos::Logos;

#[derive(Logos, Debug, PartialEq, Eq)]
pub enum Token<'a> {
    #[token("pub")]
    Pub,

    #[token("when")]
    When,

    #[token("is")]
    Is,

    #[token("else")]
    Else,

    #[token("then")]
    Then,

    #[token("func")]
    Func,

    #[token("type")]
    Type,

    #[token("use")]
    Use,

    #[token("..")]
    Spread,

    #[token("let")]
    Let,

    #[token("return")]
    Return,

    #[token("loop")]
    Loop,

    #[token("break")]
    Break,

    #[token("next")]
    Next,

    #[token("if")]
    If,

    #[token("do")]
    Do,

    #[token("for")]
    For,

    #[token("in")]
    In,

    #[token("generic")]
    Generic,

    #[token("where")]
    Where,

    #[token("mut")]
    Mut,

    // must match ([a-z][A-Z])+([a-z][A-Z][0-9])*(_([a-z][A-Z][0-9])*)*
    #[regex(r"[a-zA-Z][a-zA-Z0-9_]+")]
    Identifier(&'a str),


    #[token("(")]
    ParenOpen,

    #[token(")")]
    ParenClosing,

    #[token("{")]
    CurlyOpen,

    #[token("}")]
    CurlyClosing,

    #[token("[")]
    BracketOpen,

    #[token("]")]
    BracketClosing,

    #[token(">=")]
    GreaterOrEqual,

    #[token("<=")]
    SmallerOrEqual,

    #[token("==")]
    Equal,

    #[token("++")]
    Concat,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Multiply,

    #[token("/")]
    Divide,

    #[token("^")]
    Power,

    #[token("√")]
    Sqrt,

    #[token("%")]
    Modulo,

    #[token("&")]
    Amp,

    #[token("and")]
    And,

    #[token("or")]
    Or,

    #[token("|")]
    Abs,

    #[token("!")]
    Not,

    #[token("=")]
    Assign,

    #[token(">")]
    Greater,

    #[token("<")]
    Smaller,

    #[token("->")]
    ArrowSlim,

    #[token("=>")]
    ArrowThick,

    #[token(".")]
    Period,

    #[token(",")]
    Comma,

    #[token(":")]
    Colon,

    #[token(";")]
    SemiColon,

    #[token("lib")]
    Lib,

    #[regex(r"\d+\.\d*(e-?\d+)?")]
    Float(&'a str),

    #[regex(r"\d+")]
    Int(&'a str),

    #[regex(r"0o[0-7]+")]
    IntOct(&'a str),

    #[regex(r"0x[0-9A-F]+")]
    IntHex(&'a str),

    #[regex(r"0b[0-1]+")]
    IntBin(&'a str),

    // Strings are a little harder, since they may be interpolated
    // TODO string interpolation
    // TODO for now strings are just ' because the regex would become a mess to write
    #[regex(r"'[^']*'")]
    String(&'a str),

    #[error]
    // We can also use this variant to define whitespace,
    // whitespace we wish to skip.
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
}
