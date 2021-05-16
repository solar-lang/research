use solar_tokenizer::Token;

pub struct FullIdentifier<'a> {
    pub tokens: &'a [Token<'a>],
    pub value: Vec<Identifier<'a>>,
}

pub struct Identifier<'a> {
    pub tokens: &'a [Token<'a>],
    pub value: &'a str,
}

pub fn is_keyword(word: &str) -> bool {
    [
        "lib", "in", "let", "and", "or", "when", "when", "is", "then", "else", "return", "loop",
        "break", "next", "set", "func", "function", "use", "type"
    ]
    .contains(&word)
}



#[cfg(test)]
mod tests {
    use super::*;
    fn keyword_recognition() {
        assert!(is_keyword("let"));
        assert!(is_keyword("in"));
        assert!(is_keyword("for"));
        assert!(is_keyword("use"));
        assert!(is_keyword("type"));
        assert!(!is_keyword("x"));
        assert!(!is_keyword("y"));
        assert!(!is_keyword("point"));
    }
}
