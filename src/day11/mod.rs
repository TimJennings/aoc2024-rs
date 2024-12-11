use std::collections::HashMap;

use crate::common::string_utils::read_file_to_string;

pub fn run() {
    let input = read_file_to_string("input/day11.txt");
    let mut stones: Vec<u64> = input
        .split(" ")
        .map(|stone| stone.parse().unwrap())
        .collect();

    println!("{stones:?}");

    for iteration in 1..=25 {
        let mut next_vec: Vec<u64> = Vec::new();
        for (_index, stone) in stones.iter().enumerate() {
            let mut str_stone = stone.to_string();
            if *stone == 0 {
                next_vec.push(1);
            } else if str_stone.len() % 2 == 0 {
                let second_half = str_stone.split_off(str_stone.len() / 2);
                next_vec.push(str_stone.parse().unwrap());
                next_vec.push(second_half.parse().unwrap());
            } else {
                next_vec.push(stone * 2024);
            }
        }

        stones = next_vec;

        println!("after iteration {iteration}");
    }

    println!("count : {:?}", stones.len());
}

pub fn run2() {
    let mut memo: HashMap<(u64, u64), u64> = HashMap::new();
    let input = read_file_to_string("input/day11.txt");
    let stones: Vec<u64> = input
        .split(" ")
        .map(|stone| stone.parse().unwrap())
        .collect();

    let mut count = 0;
    for stone in stones.iter() {
        // do we have a cached response for number of stones based on blinks remaining?
        if memo.contains_key(&(*stone, 75)) {
            count = count + memo.get(&(*stone, 75)).unwrap();
        } else {
            count = count + count_stone(*stone, 75, &mut memo);
        }
    }

    // println!("{stones:?}");

    println!("count : {:?}", count);
}

fn count_stone(stone: u64, remaining_blinks: u64, memo: &mut HashMap<(u64, u64), u64>) -> u64 {
    // terminating cases, either we've seen this exact call before or we're done blinking and have one stone
    if memo.contains_key(&(stone, remaining_blinks)) {
        return *memo.get(&(stone, remaining_blinks)).unwrap();
    } else if remaining_blinks == 0 {
        return 1;
    } else {
        // blink
        let remaining_blinks = remaining_blinks - 1;
        let mut str_stone = stone.to_string();
        if stone == 0 {
            let count = count_stone(1, remaining_blinks, memo);
            memo.insert((1, remaining_blinks), count);
            return count;
        } else if str_stone.len() % 2 == 0 {
            let second_half = str_stone.split_off(str_stone.len() / 2);

            let count1 = count_stone(str_stone.parse().unwrap(), remaining_blinks, memo);
            memo.insert((str_stone.parse().unwrap(), remaining_blinks), count1);
            let count2 = count_stone(second_half.parse().unwrap(), remaining_blinks, memo);
            memo.insert((second_half.parse().unwrap(), remaining_blinks), count2);
            let count = count1 + count2;
            return count;
        } else {
            let count = count_stone(stone * 2024, remaining_blinks, memo);
            memo.insert((stone * 2024, remaining_blinks), count);
            return count;
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{common::string_utils::read_file_to_string, day11::count_stone};

    const TEST_DATA: &str = "125 17";

    #[test]
    pub fn test() {
        let mut stones: Vec<u64> = TEST_DATA
            .split(" ")
            .map(|stone| stone.parse().unwrap())
            .collect();

        println!("{stones:?}");

        for _iteration in 1..=25 {
            let mut next_vec: Vec<u64> = Vec::new();
            for (_index, stone) in stones.iter().enumerate() {
                let mut str_stone = stone.to_string();
                if *stone == 0 {
                    next_vec.push(1);
                } else if str_stone.len() % 2 == 0 {
                    let second_half = str_stone.split_off(str_stone.len() / 2);
                    next_vec.push(str_stone.parse().unwrap());
                    next_vec.push(second_half.parse().unwrap());
                } else {
                    next_vec.push(stone * 2024);
                }
            }

            stones = next_vec;

            // println!("after iteration {iteration}: {stones:?}");
        }

        println!("count : {:?}", stones.len());
    }

    #[test]
    pub fn test2() {
        let mut memo: HashMap<(u64, u64), u64> = HashMap::new();

        let input = read_file_to_string("input/day11.txt");
        // let mut stones: Vec<u64> = input
        //     .split(" ")
        //     .map(|stone| stone.parse().unwrap())
        //     .collect();

        let mut stones: Vec<u64> = TEST_DATA
            .split(" ")
            .map(|stone| stone.parse().unwrap())
            .collect();

        let mut count = 0;
        for stone in stones.iter() {
            // do we have a cached response for number of stones based on blinks remaining?
            if memo.contains_key(&(*stone, 25)) {
                count = count + memo.get(&(*stone, 25)).unwrap();
            } else {
                count = count + count_stone(*stone, 25, &mut memo);
            }
        }

        // println!("{stones:?}");

        println!("count : {:?}", count);
        // println!("{:?}", memo);
    }
}
