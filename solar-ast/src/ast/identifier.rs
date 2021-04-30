use solar_tokenizer::Token;

pub struct Identifier<'a> {
    pub tokens: &'a [Token<'a>],
    pub value: &'a str,
}
