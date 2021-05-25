use nom::IResult;

pub type Res<'a, T> = IResult<&'a str, T>;

pub trait Parse<'a>
where
    Self: Sized,
{
    fn parse(input: &'a str) -> Res<'a, Self>;

    fn parse_ws(input: &'a str) -> Res<'a, Self> {
        ws(Self::parse)(input)
    }
}

pub trait Combinator<O> {
    fn ws(self) -> O;
}

pub fn ws<'a, T>(f: impl Fn(&'a str) -> Res<'a, T>) -> impl Fn(&'a str) -> Res<'a, T> {
    move |input: &str| {
        let (input, _whitespace) =
            nom::bytes::complete::take_while(|c| c == ' ' || c == '\n')(input)?;
        f(input)
    }
}
