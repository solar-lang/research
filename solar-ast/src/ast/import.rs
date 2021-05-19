
use crate::{ parse::*, ast::*, util::* };

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Import<'a> {
    pub span: &'a str,
    pub path: ImportPath<'a>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImportPath<'a> {
    pub span: &'a str,
    pub from: identifier::Identifier<'a>,
    pub select: Option<Box<ImportSelector<'a>>>,
}

impl<'a> Parse<'a> for ImportPath<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, from) = identifier::Identifier::parse(input)?;

        match ImportSelector::parse_ws(rest) {
            Ok((rest, select)) => {
                let select = Some(Box::new(select));
                let span = unsafe { from_to(input, rest) };

                Ok((rest, ImportPath {span, from, select}))
            },
            _ => {
                let span = unsafe { from_to(input, rest) };
                Ok((rest, ImportPath { span, from, select: None}))
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ImportSelector<'a> {
    // .. (spread)
    Everything { span: &'a str },
    // .xyz
    Package(ImportPath<'a>),
    Packages(Vec<ImportPath<'a>>),
}

impl<'a> Parse<'a> for ImportSelector<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        use nom::{sequence::{delimited, preceded }, combinator::map, branch::alt, multi::many1 };
        use keywords::*;

        alt((
            map(KeywordSpread::parse, |KeywordSpread {span}| ImportSelector::Everything {span}),
            map(preceded(KeywordDot::parse_ws, ImportPath::parse_ws ), ImportSelector::Package),
            map(delimited(KeywordParenOpen::parse, many1(ImportPath::parse_ws), KeywordParenClose::parse_ws), ImportSelector::Packages),
                ))(input)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imports() {
        let input = "std.collections(hashmap vector util..)  ";
        let imports = ImportPath::parse(input);
        assert!(imports.is_ok());
        let (rest, imports) = imports.unwrap();
        assert_eq!(rest, "  ");
        assert_eq!(imports.span, &input[..( input.len() -2 )]);
    }
}

