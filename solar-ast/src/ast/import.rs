use crate::{ast::*, parse::*, util::*};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Import<'a> {
    pub span: &'a str,
    pub path: ImportPath<'a>,
}

impl<'a> Parse<'a> for Import<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, _) = keywords::Use::parse(input)?;
        let (rest, path) = ImportPath::parse_ws(rest)?;

        let span = unsafe { from_to(input, rest) };

        Ok((rest, Import { span, path }))
    }
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

                Ok((rest, ImportPath { span, from, select }))
            }
            _ => {
                let span = unsafe { from_to(input, rest) };
                Ok((
                    rest,
                    ImportPath {
                        span,
                        from,
                        select: None,
                    },
                ))
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
        use keywords::*;
        use nom::{
            branch::alt,
            combinator::map,
            multi::many1,
            sequence::{delimited, preceded},
        };

        alt((
            map(Spread::parse, |Spread { span }| {
                ImportSelector::Everything { span }
            }),
            map(
                preceded(Dot::parse_ws, ImportPath::parse_ws),
                ImportSelector::Package,
            ),
            map(
                delimited(
                    ParenOpen::parse,
                    many1(ImportPath::parse_ws),
                    ParenClose::parse_ws,
                ),
                ImportSelector::Packages,
            ),
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
        assert_eq!(imports.span, &input[..(input.len() - 2)]);
    }

    // testing full imports with `use` keyword at the start
    #[test]
    fn full_imports() {
        let input = "use std.collections.hashmap";
        let (rest, import) = Import::parse(input).unwrap();
        assert_eq!(
            import,
            Import {
                span: input,
                path: ImportPath::parse("std.collections.hashmap").unwrap().1
            }
        );
        assert_eq!(rest, "");
    }
}
