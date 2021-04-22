#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// Identifier names in solar may
// - start with alpha
// - contain _, alphanumeric
// - may not contain __
// - must end with alphanumeric

// Full identifier names (including path)
// may contain <identifier> separated by `.`

// Plain identifiers may contain
// - Solaridentifiers
// - < > ( ) ,
//   ^-------^ in order to cotain generics and tuples

// special character: j
// (e.g. j must be escaped)

// escape
// j -> jj
// . -> __
// < -> ja
// > -> jb
// ( -> jc
// ) -> jd
// , -> je

pub fn mangle(identifier: &str) -> String {
    let mut buf = String::with_capacity(identifier.len() * 2);
    for c in identifier.chars() {
        match c {
            '.' => buf.push_str("__"),
            'j' => buf.push_str("jj"),
            '<' => buf.push_str("ja"),
            '>' => buf.push_str("jb"),
            '(' => buf.push_str("jc"),
            ')' => buf.push_str("jd"),
            ',' => buf.push_str("je"),
            _ => buf.push(c),
        }
    }

    buf
}

pub fn demangle(identifier: &str) -> Option<String> {
    if identifier.len() == 0 {
        return Some(String::new());
    }

    if identifier.starts_with("__") {
        let i = &identifier[2..];
        return Some(format!(".{}", demangle(i)?));
    }

    if identifier.starts_with('j') {
        let identifier = &identifier[1..];
        let c = match identifier.chars().next()? {
            'j' => 'j',
            'a' => '<',
            'b' => '>',
            'c' => '(',
            'd' => ')',
            'e' => ',',
            _ => return None,
        };

        let identifier = &identifier[1..];
        return Some(format!("{}{}", c, demangle(identifier)?));
    }

    Some(format!(
        "{}{}",
        identifier.chars().next().unwrap(),
        // Note: may break when next char is non ascii
        &identifier[1..]
    ))
}
