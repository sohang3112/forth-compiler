use std::str::FromStr;
use nom::{
    Err, Finish, IResult, Parser,
    branch::alt,
    combinator::{map, map_res, eof},
    number::{i32, f32},
    sequence::terminated
};

pub enum Number {
    Int(i32),
    Float(f32)
}
pub enum AST<'a> {
    Push(Number),
    BuiltinWord(&'a str),
}

fn number(input: &str) -> IResult<&str, Number> {
    // alt((
    //     map(i32, Number::Int),
    //     map(f32, Number::Float),
    // ))(input)
    // map(i32, Number::Int)(input)
    i32(input)
}

fn program(input: &str) -> IResult<&str, AST> {
    terminated(
        map(number, AST::Push), 
        eof
    )(input)
}

impl FromStr for AST {
    type Err = nom::error::Error<&'a str>;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        let (_, ir) = program(code).finish()?;
        Ok(ir)
    }
}