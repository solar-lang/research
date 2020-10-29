use crate::Span;

pub fn ws(s: Span) -> nom::IResult<Span, Span> {
    use nom::bytes::complete::take_while;

    take_while(|c| c == ' ' || c == '\r' || c == '\t' || c == '\n')(s)
}

pub mod characters {
    #[inline(always)]
    pub fn alpha(c: char) -> bool {
        c >= 'a' && c <= 'z' || c >= 'a' && c <= 'Z'
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
}
