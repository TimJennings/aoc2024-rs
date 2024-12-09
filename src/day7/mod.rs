use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space1},
    combinator::opt,
    multi::{many0, separated_list1},
    sequence::{terminated, tuple},
    IResult,
};

use crate::common::{self, nom_utils::parser_u64, string_utils::read_file_to_string};

pub fn run() {
    let input = common::string_utils::read_file_to_string("input/day7.txt");
    let p = parse_equations(&input);
    println!("{:?}", p);

    let equations = p.unwrap().1;

    let mut calibration_result = 0;
    for (total, numbers) in equations.iter() {
        if can_total(*total, 0, numbers) {
            calibration_result = calibration_result + total;
        }
    }
    println!("result: {}", calibration_result);
}

pub fn run2() {
    let input = common::string_utils::read_file_to_string("input/day7.txt");
    let p = parse_equations(&input);
    println!("{:?}", p);

    let equations = p.unwrap().1;

    let mut calibration_result = 0;
    for (total, numbers) in equations.iter() {
        if can_total2(*total, 0, numbers) {
            calibration_result = calibration_result + total;
        }
    }
    println!("result: {}", calibration_result);
}

fn parse_equations(input: &str) -> IResult<&str, (Vec<(u64, Vec<u64>)>)> {
    let p = many0(terminated(
        tuple((parser_u64, tag(": "), separated_list1(space1, parser_u64))),
        opt(line_ending),
    ))(&input);
    match p {
        Ok((remaining, v)) => {
            let mut result = Vec::new();
            for line in v.iter() {
                result.push((line.0, line.2.clone()));
            }
            Ok((remaining, result))
        }
        Err(e) => Err(e),
    }
}

fn can_total(target: u64, current: u64, remaining_input: &[u64]) -> bool {
    //pop off the next input and attempt + and *, if we're over return false, if we're out of input and at target return true
    let first = remaining_input.split_first();
    match first {
        Some((first, rest)) => {
            return can_total(target, current + first, rest)
                || can_total(target, current * first, rest);
        }
        None => {
            if target == current {
                println!("Found target");
                return true;
            }
            return false;
        }
    }
}

fn can_total2(target: u64, current: u64, remaining_input: &[u64]) -> bool {
    //pop off the next input and attempt + and *, if we're over return false, if we're out of input and at target return true
    let first = remaining_input.split_first();
    if current > target {
        return false;
    }
    match first {
        Some((first, rest)) => {
            return can_total2(target, current + first, rest)
                || can_total2(target, current * first, rest)
                || can_total2(
                    target,
                    (current.to_string() + &first.to_string()).parse().unwrap(),
                    rest,
                );
        }
        None => {
            if target == current {
                println!("Found target");
                return true;
            }
            return false;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::day7::{can_total, can_total2};

    use super::parse_equations;

    const TEST_DATA: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    pub fn test() {
        let p = parse_equations(&TEST_DATA);
        println!("{:?}", p);

        let equations = p.unwrap().1;

        let mut calibration = 0;
        for (total, numbers) in equations.iter() {
            if can_total(*total, 0, &numbers) {
                calibration = calibration + total;
            }
        }
        assert_eq!(3749, calibration);
    }

    #[test]
    pub fn test2() {
        let p = parse_equations(&TEST_DATA);
        println!("{:?}", p);

        let equations = p.unwrap().1;

        let mut calibration = 0;
        for (total, numbers) in equations.iter() {
            if can_total2(*total, 0, &numbers) {
                calibration = calibration + total;
            }
        }

        // assert_eq!(true, can_total2(156, 0, &[15, 6]));
        assert_eq!(11387, calibration);
    }
}
