use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::map,
    multi::many0,
    sequence::tuple,
    IResult,
};

use crate::common::{self, nom_utils::parser_u64};

pub fn run() {
    let input = common::string_utils::read_file_to_string("input/day3.txt");
    let muls = many0(parse_mul)(&input).unwrap();

    let mut sum = 0;
    let mut enabled = true;
    for mul in muls.1 {
        match mul {
            Token::Char(_) => continue,
            Token::Do(_) => {
                enabled = true;
                continue;
            }
            Token::Dont(_) => {
                enabled = false;
                continue;
            }
            Token::Mul(left, right) => {
                if enabled {
                    sum = sum + (left * right);
                }
            }
        }
    }
    println!("count is :{}", sum);
}

#[derive(Debug)]
enum Token<'a> {
    Char(&'a str),
    Mul(u64, u64),
    Do(&'a str),
    Dont(&'a str),
}
fn parse_mul(input: &str) -> IResult<&str, Token> {
    alt((
        map(tag("do()"), Token::Do),
        map(tag("don't()"), Token::Dont),
        map(
            tuple((tag("mul("), parser_u64, tag(","), parser_u64, tag(")"))),
            |t| Token::Mul(t.1, t.3),
        ),
        map(take(1usize), Token::Char),
    ))(input)
}

#[cfg(test)]
mod test {
    use nom::multi::many0;

    use super::parse_mul;

    #[test]
    pub fn test() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let mul = many0(parse_mul)(input).unwrap();
        println!("{:?}", mul);
    }

    #[test]
    pub fn test2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let mul = many0(parse_mul)(input).unwrap();
        println!("{:?}", mul);
    }
}
