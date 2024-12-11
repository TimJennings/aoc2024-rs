use std::{env, time::Instant};

mod common;
mod day1;
mod day10;
mod day11;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = if args.len() >= 2 { &args[1] } else { "11" };

    println!("Running day {day}");
    let now = Instant::now();
    match day {
        "1" => {
            day1::run();
        }
        "2" => {
            day2::run();
        }
        "3" => {
            day3::run();
        }
        "4" => {
            day4::run();
            day4::run2();
        }
        "5" => {
            day5::run();
            day5::run2();
        }
        "6" => {
            day6::run();
            day6::run2();
        }
        "6.2" => {
            day6::run2();
        }
        "7" => {
            day7::run();
            day7::run2();
        }
        "9" => {
            day9::run();
            day9::run2();
        }
        "10" => {
            day10::run();
            day10::run2();
        }
        "11" => {
            day11::run();
            day11::run2();
        }
        _ => {}
    }
    let elapsed_time = now.elapsed();
    println!("Running took {} milliseconds.", elapsed_time.as_millis());
    println!("Running took {} nanoseconds.", elapsed_time.as_nanos());
}
