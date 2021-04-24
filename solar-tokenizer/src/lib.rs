use logos::Logos;

#[cfg(test)]
mod tests {
    use super::Token;
    use super::Token::*;

    fn tokenize(input: &str) -> Vec<Token> {
        super::only_relevant(super::tokenize(input)).collect()
    }

    #[test]
    fn tokens1() {
        let input = "hello world";
        let expected = [Identifier("hello"), Identifier("world")];

        assert_eq!(&tokenize(input), &expected);
    }

    #[test]
    fn tokens3() {
        let input = "# this is a comment \n map f";
        let expected = [
            Comment("# this is a comment "),
            Identifier("map"),
            Identifier("f"),
        ];

        assert_eq!(&tokenize(input), &expected);
    }

    #[test]
    fn tokens2() {
        let input = "pub func map(list List T, fn T -> R) -> List R";
        let expected = [
            Pub("pub"),
            Func("func"),
            Identifier("map"),
            ParenOpen("("),
            Identifier("list"),
            Identifier("List"),
            Identifier("T"),
            Comma(","),
            Identifier("fn"),
            Identifier("T"),
            ArrowSlim("->"),
            Identifier("R"),
            ParenClosing(")"),
            ArrowSlim("->"),
            Identifier("List"),
            Identifier("R"),
        ];

        assert_eq!(&tokenize(input), &expected);
    }
}

#[derive(Logos, Debug, PartialEq, Eq)]
pub enum Token<'a> {
    #[regex(r"#[^\n]*")]
    Comment(&'a str),

    #[token("pub")]
    Pub(&'a str),

    #[token("test")]
    Test(&'a str),

    #[token("when")]
    When(&'a str),

    #[token("is")]
    Is(&'a str),

    #[token("else")]
    Else(&'a str),

    #[token("then")]
    Then(&'a str),

    #[token("func")]
    Func(&'a str),

    #[token("type")]
    Type(&'a str),

    #[token("use")]
    Use(&'a str),

    #[token("..")]
    Spread(&'a str),

    #[token("let")]
    Let(&'a str),

    #[token("return")]
    Return(&'a str),

    #[token("loop")]
    Loop(&'a str),

    #[token("break")]
    Break(&'a str),

    #[token("next")]
    Next(&'a str),

    #[token("if")]
    If(&'a str),

    #[token("do")]
    Do(&'a str),

    #[token("for")]
    For(&'a str),

    #[token("in")]
    In(&'a str),

    #[token("generic")]
    Generic(&'a str),

    #[token("where")]
    Where(&'a str),

    #[token("mut")]
    Mut(&'a str),

    // Variables, types and directories must match ([a-z][A-Z])+([a-z][A-Z][0-9])*(_([a-z][A-Z][0-9])*)*
    // Though it's possoble to have identifiers coming from `c`
    #[regex(r"[a-zA-Z][a-zA-Z0-9_]*")]
    Identifier(&'a str),

    #[token("(")]
    ParenOpen(&'a str),

    #[token(")")]
    ParenClosing(&'a str),

    #[token("{")]
    CurlyOpen(&'a str),

    #[token("}")]
    CurlyClosing(&'a str),

    #[token("[")]
    BracketOpen(&'a str),

    #[token("]")]
    BracketClosing(&'a str),

    #[token(">=")]
    GreaterOrEqual(&'a str),

    #[token("<=")]
    SmallerOrEqual(&'a str),

    #[token("==")]
    Equal(&'a str),

    #[token("++")]
    Concat(&'a str),

    #[token("+")]
    Plus(&'a str),

    #[token("-")]
    Minus(&'a str),

    #[token("*")]
    Multiply(&'a str),

    #[token("/")]
    Divide(&'a str),

    #[token("^")]
    Power(&'a str),

    #[token("√")]
    Sqrt(&'a str),

    #[token("%")]
    Modulo(&'a str),

    #[token("&")]
    Amp(&'a str),

    #[token("and")]
    And(&'a str),

    #[token("or")]
    Or(&'a str),

    #[token("|")]
    Abs(&'a str),

    #[token("!")]
    Not(&'a str),

    #[token("=")]
    Assign(&'a str),

    #[token(">")]
    Greater(&'a str),

    #[token("<")]
    Smaller(&'a str),

    #[token("->")]
    ArrowSlim(&'a str),

    #[token("=>")]
    ArrowThick(&'a str),

    #[token(".")]
    Period(&'a str),

    #[token(",")]
    Comma(&'a str),

    #[token(":")]
    Colon(&'a str),

    #[token(";")]
    SemiColon(&'a str),

    #[token("lib")]
    Lib(&'a str),

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
    // We can also use this variant to define whitespace(&'a str),
    // whitespace we wish to skip.
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
}

/// tokenize input string format of solar language
pub fn tokenize(input: &str) -> impl Iterator<Item = Token> {
    Token::lexer(input)
}

/// filter out all syntactically irrelevant tokens
pub fn only_relevant<'a>(
    tokens: impl Iterator<Item = Token<'a>>,
) -> impl Iterator<Item = Token<'a>> {
    tokens.filter(not_whitespace).filter(not_comment)
}

fn not_whitespace(t: &Token) -> bool {
    t != &Token::Whitespace
}

fn not_comment(t: &Token) -> bool {
    match t {
        &Token::Comment(_) => false,
        _ => true,
    }
}
