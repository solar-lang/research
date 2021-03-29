mod token;
mod ast;

peg::parser! {
  grammar list_parser() for str {
    rule number() -> u32
      = n:$(['0'..='9']+) { n.parse().unwrap() }

    pub rule list() -> Vec<u32>
      = "[" l:number() ** "," "]" { l }
  }
}

peg::parser! {
    grammar solar() for str {
        /// whitespaces. Note, solar does not handle tabs
        rule _() = [' ' | '\n']

        /// one whitespace character or end of file
        rule stop() = [' ' | '\n'] / ![_]

        pub rule function_call() -> FunctionCall<'input> =
            name:ident() (
                _

            )

        pub rule ident() -> &'input str
            = s:$(['a'..='z' | 'A'..='Z']+ ['a'..='z' | 'A'..='Z' | '0'..='9']* ("_" ['a'..='z' | 'A'..='Z' | '0'..='9']+)*) { s }

        pub rule number() -> &'input str
            = s:$(
                [ '0'..='9' ]+ ("." ['0'..='9']* ("-"? "e" ['0'..='9']+)? )?
                // / ("0x" ['0..='9' | 'a'..='f' | 'A'..='F']+)
                // / ("0o" ['0'..='7'])
                // / ("0b" ['0' | '1' ])
                ) { s }

        pub rule function () -> Function<'input> =
            "function" _+ name:ident() _* "(" args:(ident() ** ( _* "," _* )) ")"
            { Function {name, args} }


    }
}

pub enum LiteralExpression<'a> {
    Bool(&'a str),
    Number(&'a str),
    String(&'a str),
    // variable or function reference
    FunctionReference(&'a str),
    // ExprInParenthesis(Expression<'a>),
    // AbsExpression(Expression<'a>),
    // Tuple(Vec<Expression<'a>>),
    // Array(Vec<Expression<'a>>),
}

pub struct Argument<'a> {
    pub name: Option<&'a str>,
    pub expr: LiteralExpression<'a>,
}

pub struct FunctionCall<'a> {
    pub name: &'a str,
    pub arguments: Vec<Argument<'a>>,
}

pub struct Function<'a> {
    pub name: &'a str,
    pub args: Vec<&'a str>,
}
