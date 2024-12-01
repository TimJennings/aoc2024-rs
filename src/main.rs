use std::env;

mod common;
mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = if args.len() >= 2 { &args[1] } else { "1" };

    println!("Running day {day}");

    match day {
        "1" => {
            day1::run();
        }
        _ => {}
    }
}
