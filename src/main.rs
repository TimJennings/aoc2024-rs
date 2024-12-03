use std::{env, time::Instant};

mod common;
mod day1;
mod day2;
mod day3;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = if args.len() >= 2 { &args[1] } else { "3" };

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
        _ => {}
    }
    let elapsed_time = now.elapsed();
    println!("Running took {} milliseconds.", elapsed_time.as_millis());
    println!("Running took {} nanoseconds.", elapsed_time.as_nanos());
}
