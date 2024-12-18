use std::cmp::{max, min};

use nom::{
    character::complete::{line_ending, space1},
    combinator::opt,
    multi::{fold_many0, many0},
    sequence::{terminated, tuple},
    IResult,
};

use crate::common::{nom_utils::parser_u64, string_utils::read_file_to_string};

struct ListPair {
    left: Vec<u64>,
    right: Vec<u64>,
}

impl ListPair {
    pub fn new() -> ListPair {
        ListPair {
            left: Vec::new(),
            right: Vec::new(),
        }
    }
}

pub fn run() {
    let input = read_file_to_string("input/day1.txt");
    let (_remaining_input, list_pair) = fold_many0(
        pair_location_id,
        ListPair::new,
        |mut acc: ListPair, item| {
            acc.left.push(item.0);
            acc.right.push(item.1);
            acc
        },
    )(&input)
    .unwrap();
    let mut left_list = list_pair.left;
    let mut right_list = list_pair.right;

    left_list.sort();
    right_list.sort();

    let mut index = 0;
    let mut result = 0;
    while index < left_list.len() {
        result = result
            + (max(left_list[index], right_list[index]) - min(left_list[index], right_list[index]));
        index = index + 1;
    }

    println!("part1: {}", result);

    index = 0;
    result = 0;
    while index < left_list.len() {
        // count number of times this appears in right list
        let left_value = left_list[index];
        let mut count = 0;
        for value in &right_list {
            if *value == left_value {
                count = count + 1;
            }
        }

        result = result + (left_value * count);
        index = index + 1;
    }

    println!("part2: {}", result);
}

fn pair_location_id(input: &str) -> IResult<&str, (u64, u64)> {
    let (input, (left, _space, right)) =
        terminated(tuple((parser_u64, space1, parser_u64)), opt(line_ending))(input)?;
    Ok((input, (left, right)))
}

#[cfg(test)]
mod test {
    use common::string_utils::read_file_to_string;
    use nom::multi::fold_many0;

    use crate::{common, day1::*};

    #[test]
    pub fn test_one_line() {
        let res = pair_location_id("40885   43247").unwrap();
        assert_eq!(res.1 .0, 40885);
        assert_eq!(res.1 .1, 43247);
    }

    #[test]
    pub fn test_full_file() {
        let input = read_file_to_string("input/day1.txt");
        let (_remaining_input, pairs) = many0(pair_location_id)(input.as_str()).unwrap();

        assert_eq!(pairs[0].0, 40885);
        assert_eq!(pairs[0].1, 43247);
        assert_eq!(pairs[1].0, 14780);
        assert_eq!(pairs[1].1, 86274);
    }

    #[test]
    pub fn test_fold() {
        let input = read_file_to_string("input/day1.txt");
        let (remaining_input, list_pair) = fold_many0(
            pair_location_id,
            ListPair::new,
            |mut acc: ListPair, item| {
                acc.left.push(item.0);
                acc.right.push(item.1);
                acc
            },
        )(&input)
        .unwrap();
        assert_eq!("", remaining_input);
        assert_eq!(1000, list_pair.left.len());
        assert_eq!(1000, list_pair.right.len());
    }
}
