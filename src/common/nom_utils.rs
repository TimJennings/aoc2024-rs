use nom::{character::complete::digit1, combinator::map_res, IResult};

pub fn parser_u64(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}
