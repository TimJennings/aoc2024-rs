use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt, recognize},
    sequence::preceded,
    IResult,
};

pub fn parser_u64(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

pub fn parser_i64(input: &str) -> IResult<&str, i64> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s: &str| {
        s.parse()
    })(input)?;

    Ok((i, number))
}
