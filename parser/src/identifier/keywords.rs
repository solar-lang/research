///
/// Keywords
///
/// Solar contains several keywords.

pub fn is_keyword(input: &str) -> bool {
    vec![
        "let", "for", "while", "if", "else", "then", "function", "return", "break", "match",
        "type", "is", "or", "in", "const", //not yet planned to be used after this
        "async",
    ]
    .contains(&input)
}
