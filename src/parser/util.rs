use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::error::ParseError;
use nom::sequence::tuple;
use nom::AsChar;
use nom::IResult;
use nom::InputTakeAtPosition;

pub fn ws<I, O, E: ParseError<I>, F>(f: F) -> impl Fn(I) -> IResult<I, O, E>
where
    I: InputTakeAtPosition + Clone,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
    E: ParseError<I>,
    F: Fn(I) -> IResult<I, O, E>,
{
    map(tuple((multispace0, f, multispace0)), |(_, res, _)| res)
}
