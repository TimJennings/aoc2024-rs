use itertools::Itertools;
use nom::{
    character::complete::{line_ending, space1},
    combinator::opt,
    multi::{many0, separated_list1},
    sequence::terminated,
    IResult,
};

use crate::common::{self, nom_utils::parser_i64};

pub fn run() {
    let input = common::string_utils::read_file_to_string("input/day2.txt");
    let reports = many0(parse_report)(&input).unwrap().1;

    // count safe reports
    let mut safe_count = 0;
    for report in reports {
        if report.is_safe() {
            safe_count = safe_count + 1;
        }
    }

    println!("Safe: {}", safe_count);
}

fn parse_report(input: &str) -> IResult<&str, Report> {
    let (remaining_input, levels) =
        terminated(separated_list1(space1, parser_i64), opt(line_ending))(input)?;

    let mut report = Report::new();
    report.levels = levels;
    Ok((remaining_input, report))
}

struct Report {
    levels: Vec<i64>,
}

enum LevelState {
    INCREASING,
    DECREASING,
}

fn is_safe_second_chance(levels: &Vec<i64>) -> bool {
    let first = levels[0];
    let second = levels[1];

    let level_state;
    if first == second {
        // neither increasing nor decreasing
        return false;
    } else if second < first {
        level_state = LevelState::DECREASING
    } else {
        level_state = LevelState::INCREASING
    }

    // safe if only changing by 1-3
    let mut index = 1;
    while index < levels.len() {
        // compare to previous
        let previous = levels[index - 1];
        let current = levels[index];
        match level_state {
            LevelState::INCREASING => {
                if current - previous <= 3 && current - previous >= 1 {
                    // safe
                } else {
                    // not safe
                    return false;
                }
            }
            LevelState::DECREASING => {
                if previous - current <= 3 && previous - current >= 1 {
                    // safe
                } else {
                    // not safe
                    return false;
                }
            }
        }
        index = index + 1;
    }
    // all the way to the end without an unsafe
    true
}

impl Report {
    pub fn new() -> Report {
        Report { levels: Vec::new() }
    }

    pub fn is_safe(self) -> bool {
        let first_try = is_safe_second_chance(&self.levels);
        if first_try {
            return true;
        } else {
            // try removing each value
            let len = self.levels.len();
            return self
                .levels
                .into_iter()
                .combinations(len - 1)
                .any(|l| is_safe_second_chance(&l));
        }
    }
}

#[cfg(test)]
mod test {
    use itertools::Itertools;
    use nom::multi::many0;

    use crate::{common, day2::parse_report};

    #[test]
    pub fn test_one_line() {
        let res = parse_report("7 6 4 2 1").unwrap();
        assert_eq!(res.1.levels.len(), 5);
        assert_eq!(res.1.levels[0], 7);
        assert_eq!(res.1.levels[4], 1);
        assert_eq!(res.1.is_safe(), true);
    }

    #[test]
    pub fn test_parse_all() {
        let input = common::string_utils::read_file_to_string("input/day2.txt");
        let reports = many0(parse_report)(&input);
        assert_eq!(reports.unwrap().1.len(), 1000);
    }

    #[test]
    pub fn test_unsafe() {
        assert_eq!(parse_report("7 6 4 2 1").unwrap().1.is_safe(), true);
        assert_eq!(parse_report("1 2 7 8 9").unwrap().1.is_safe(), false);
        assert_eq!(parse_report("9 7 6 2 1").unwrap().1.is_safe(), false);
        assert_eq!(parse_report("1 3 2 4 5").unwrap().1.is_safe(), true);
        assert_eq!(parse_report("8 6 4 4 1").unwrap().1.is_safe(), true);
        assert_eq!(parse_report("1 3 6 7 9").unwrap().1.is_safe(), true);
        assert_eq!(parse_report("1 1 2 3 4 5").unwrap().1.is_safe(), true);

        assert_eq!(
            parse_report("48 46 47 49 51 54 56").unwrap().1.is_safe(),
            true
        );
        assert_eq!(parse_report("1 1 2 3 4 5").unwrap().1.is_safe(), true);
        assert_eq!(parse_report("1 2 3 4 5 5").unwrap().1.is_safe(), true);
        assert_eq!(parse_report("5 1 2 3 4 5").unwrap().1.is_safe(), true);
        assert_eq!(parse_report("1 4 3 2 1").unwrap().1.is_safe(), true);
        assert_eq!(parse_report("1 6 7 8 9").unwrap().1.is_safe(), true);
        assert_eq!(parse_report("1 2 3 4 3").unwrap().1.is_safe(), true);
        assert_eq!(parse_report("9 8 7 6 7").unwrap().1.is_safe(), true);
        assert_eq!(parse_report("7 10 8 10 11").unwrap().1.is_safe(), true);
        assert_eq!(
            parse_report("29 28 27 25 26 25 22 20").unwrap().1.is_safe(),
            true
        );
    }

    #[test]
    pub fn test_safe() {
        assert_eq!(
            parse_report("48 46 47 49 51 54 56").unwrap().1.is_safe(),
            true
        );
        let value = parse_report("48 46 47 49 51 54 56")
            .unwrap()
            .1
            .levels
            .into_iter()
            .combinations(6);
        for value in value {
            println!("{:?}", value);
        }
    }
}
