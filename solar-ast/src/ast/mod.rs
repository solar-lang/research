pub mod body;
pub mod expr;
pub mod identifier;
pub mod import;
pub mod keywords;
pub mod type_signature;
use body::FunctionOrTypeOrTest;
use import::Import;

/// Tree representation of the syntax of a solar file
pub struct Ast<'a> {
    pub span: &'a str,
    pub imports: Vec<Import<'a>>,
    pub functions_and_types_and_tests: Vec<FunctionOrTypeOrTest<'a>>,
}

impl<'a> crate::parse::Parse<'a> for Ast<'a> {
    fn parse(input: &'a str) -> crate::parse::Res<'a, Self> {
        use nom::multi::many0;

        let (rest, imports) = many0(Import::parse_ws)(input)?;
        let (rest, functions_and_types_and_tests) = many0(FunctionOrTypeOrTest::parse_ws)(rest)?;

        let span = unsafe { crate::util::from_to(input, rest) };

        Ok((
            rest,
            Ast {
                span,
                imports,
                functions_and_types_and_tests,
            },
        ))
    }
}
