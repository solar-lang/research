#[inline(always)]
pub fn alpha(c: char) -> bool {
    c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z'
}

#[inline(always)]
pub fn digit10(c: char) -> bool {
    c >= '0' && c <= '9'
}

#[inline(always)]
pub fn digit16(c: char) -> bool {
    c >= '0' && c <= 'F'
}

#[inline(always)]
pub fn digit8(c: char) -> bool {
    c >= '0' && c <= '8'
}

#[inline(always)]
pub fn digit2(c: char) -> bool {
    c == '0' || c == '1'
}

#[inline(always)]
pub fn alphanumeric(c: char) -> bool {
    alpha(c) || digit10(c)
}

#[inline(always)]
pub fn identchar(c: char) -> bool {
    alpha(c) || digit10(c) || c == '_'
}

#[inline(always)]
pub fn whitespace(c: char) -> bool {
    c == ' ' || c == '\n' || c == '\r' || c == '\t'
}
