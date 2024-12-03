use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::map,
    multi::{fold_many0, many0},
    sequence::tuple,
    IResult,
};

use crate::common::{self, nom_utils::parser_u64};

#[derive(Debug)]
struct State {
    count: u64,
    enabled: bool,
}

impl State {
    pub fn new() -> Self {
        State {
            count: 0,
            enabled: true,
        }
    }
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

pub fn run() {
    let input = common::string_utils::read_file_to_string("input/day3.txt");
    let state = fold_many0(parse_mul, State::new, |mut acc: State, token: Token| {
        match token {
            Token::Char(_) => {}
            Token::Do(_) => {
                acc.enabled = true;
            }
            Token::Dont(_) => {
                acc.enabled = false;
            }
            Token::Mul(left, right) => {
                if acc.enabled {
                    acc.count = acc.count + (left * right);
                }
            }
        }
        acc
    })(&input)
    .unwrap();
    println!("count is {}", state.1.count);
}

#[cfg(test)]
mod test {
    use nom::multi::{fold_many0, many0};

    use crate::common;

    use super::{parse_mul, State, Token};

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

    #[test]
    pub fn fold() {
        let input = common::string_utils::read_file_to_string("input/day3.txt");

        let state = fold_many0(parse_mul, State::new, |mut acc: State, token: Token| {
            match token {
                Token::Char(_) => {}
                Token::Do(_) => {
                    acc.enabled = true;
                }
                Token::Dont(_) => {
                    acc.enabled = false;
                }
                Token::Mul(left, right) => {
                    if acc.enabled {
                        acc.count = acc.count + (left * right);
                    }
                }
            }
            acc
        })(&input)
        .unwrap();
        println!("{:?}", state);
    }
}
