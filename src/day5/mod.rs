use std::cmp::Ordering;

use nom::{
    bytes::complete::tag,
    character::complete::line_ending,
    multi::{many0, many1, separated_list0},
    sequence::{terminated, tuple},
    IResult,
};

use crate::common::{self, nom_utils::parser_u64};

pub fn run() {
    let input = common::string_utils::read_file_to_string("input/day5.txt");
    let (remaining, rules) = many1(parse_order)(&input).unwrap();
    let (_remaining, updates) = parse_update(&remaining).unwrap();

    let valid_updates = get_valid_updates(&updates, &rules);
    let mut count_middle_values = 0;
    for update in valid_updates.iter() {
        //count the middle element
        let mid = (update.len() / 2);
        let count = update[mid];
        count_middle_values = count_middle_values + count;
    }
    println!("count: {} ", count_middle_values);
}

pub fn run2() {
    let input = common::string_utils::read_file_to_string("input/day5.txt");
    let (remaining, rules) = many1(parse_order)(&input).unwrap();
    let (_remaining, updates) = parse_update(&remaining).unwrap();

    let mut invalid_updates = get_invalid_updates(&updates, &rules);

    for update in invalid_updates.iter_mut() {
        update.sort_by(|a, b| {
            let afters = get_must_appear_after(*a, &rules);
            if afters.contains(b) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
    }

    let mut count_middle_values = 0;
    for update in invalid_updates.iter() {
        //count the middle element
        let mid = (update.len() / 2);
        let count = update[mid];
        count_middle_values = count_middle_values + count;
    }
    println!("count: {} ", count_middle_values);
}

fn get_valid_updates(updates: &Vec<Vec<u64>>, rules: &Vec<(u64, u64)>) -> Vec<Vec<u64>> {
    let mut result = Vec::new();
    for update in updates.iter() {
        let broken = update_is_broken(update, rules);

        if !broken {
            result.push(update.clone());
        }
    }
    result
}

fn get_invalid_updates(updates: &Vec<Vec<u64>>, rules: &Vec<(u64, u64)>) -> Vec<Vec<u64>> {
    let mut result = Vec::new();
    for update in updates.iter() {
        let broken = update_is_broken(update, rules);

        if broken {
            result.push(update.clone());
        }
    }
    result
}

fn update_is_broken(update: &Vec<u64>, rules: &Vec<(u64, u64)>) -> bool {
    let mut broken = false;
    for (index, value) in update.iter().enumerate() {
        // get list of numbers value must appear before, then check if any of those are in the rest of this list (index to end), which disqualifies this update
        if index == update.len() - 1 {
            // we're at the end, safe
            return broken;
        }
        let befores = get_must_appear_after(*value, rules);

        let after_slice = &update[index + 1..update.len()];

        for before in befores.iter() {
            for after in after_slice.iter() {
                if *before == *after {
                    // rule broken
                    // println!("{} cannot appear after {} in {:?}", value, before, update);
                    broken = true;
                    return broken;
                }
            }
        }
    }
    broken
}

fn get_must_appear_after(value: u64, rules: &Vec<(u64, u64)>) -> Vec<u64> {
    let mut result = Vec::new();
    for (before, after) in rules.iter() {
        if *after == value {
            result.push(*before);
        }
    }
    result
}

fn parse_order(input: &str) -> IResult<&str, (u64, u64)> {
    let result = terminated(tuple((parser_u64, tag("|"), parser_u64)), line_ending)(&input);

    match result {
        Ok((remaining, (l, _, r))) => Ok((remaining, (l, r))),
        Err(E) => Err(E),
    }
}

fn parse_update(input: &str) -> IResult<&str, Vec<Vec<u64>>> {
    let result = tuple((
        line_ending,
        many0(terminated(
            separated_list0(tag(","), parser_u64),
            line_ending,
        )),
    ))(&input);

    match result {
        Ok((remaining, (_nl, list))) => Ok((remaining, list)),
        Err(E) => Err(E),
    }
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering;

    use itertools::Itertools;
    use nom::multi::many1;

    use crate::{
        common,
        day5::{get_invalid_updates, get_valid_updates, parse_update, update_is_broken},
    };

    use super::{get_must_appear_after, parse_order};

    const TEST_DATA: &str = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    pub fn test_parse() {
        let (remaining, rules) = many1(parse_order)(&TEST_DATA).unwrap();
        let (remaining, updates) = parse_update(&remaining).unwrap();

        println!("remaining: {}", remaining);
        println!("rules: {:?}", rules);
        println!("updates: {:?}", updates);

        let valid_updates = get_valid_updates(&updates, &rules);
        println!("valid updates {:?}", valid_updates);
    }

    #[test]
    pub fn test_2() {
        let (remaining, rules) = many1(parse_order)(&TEST_DATA).unwrap();
        let (_remaining, updates) = parse_update(&remaining).unwrap();

        let mut invalid_updates = get_invalid_updates(&updates, &rules);

        for update in invalid_updates.iter_mut() {
            update.sort_by(|a, b| {
                let afters = get_must_appear_after(*a, &rules);
                if afters.contains(b) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
        }

        let mut count_middle_values = 0;
        for update in invalid_updates.iter() {
            //count the middle element
            let mid = (update.len() / 2);
            let count = update[mid];
            count_middle_values = count_middle_values + count;
        }

        assert_eq!(123, count_middle_values);
    }
}
