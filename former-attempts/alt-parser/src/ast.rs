use crate::token::Token;

pub enum Expression<'a> {
    Bool(Bool),
    Int(Int),
    Float(Float),
    String(String),
    Array(Array<'a>),
}

pub struct Bool(bool);

pub struct Int {
    base: u8
}

pub struct Float; // {value, exponent}

//
// TODO string interpolation
pub struct StringValue {
    pub value: String,
}

pub struct Array<'a> {
    values: Vec<Token<'a, Expression<'a>>>,
}
