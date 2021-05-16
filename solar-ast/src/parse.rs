use crate::{Error, Res};
use solar_tokenizer::Token;

pub trait Parse<'a>
where
    Self: Sized,
{
    fn parse(tokens: &'a [Token<'a>]) -> Res<'a, Self>;
}

pub fn many<'a, T>(
    f: impl Fn(&'a [Token<'a>]) -> Res<'a, T>,
    n: usize,
) -> impl Fn(&'a [Token<'a>]) -> Res<'a, Vec<T>> {
    move |mut input| {
        let mut values = Vec::new();

        let mut left_todo = n;

        loop {
            match f(input) {
                Err(e) => {
                    if left_todo > 0 {
                        return Err(e);
                    }
                    break;
                }
                Ok((value, rest)) => {
                    if left_todo > 0 {
                        left_todo -= 1;
                    }

                    values.push(value);
                    input = rest;
                }
            }
        }

        Ok((values, input))
    }
}

pub fn join<'a, T, Ignored>(
    f: impl Fn(&'a [Token<'a>]) -> Res<'a, T>,
    separator: impl Fn(&'a [Token<'a>]) -> Res<'a, Ignored>,
    n: usize,
) -> impl Fn(&'a [Token<'a>]) -> Res<'a, Vec<T>> {
    move |mut input| {
        let mut values = Vec::new();

        let mut left_todo = n;

        loop {
            match f(input) {
                Err(e) => {
                    if left_todo > 0 {
                        return Err(e);
                    }
                    break;
                }
                Ok((value, rest)) => {
                    if left_todo > 0 {
                        left_todo -= 1;
                    }

                    values.push(value);
                    input = rest;
                }
            }

            match separator(input) {
                Err(e) => {
                    if left_todo > 0 {
                        return Err(e);
                    }
                },
                Ok((_, rest)) => {
                    input = rest;
                }
            }
        }

        Ok((values, input))
    }

}
