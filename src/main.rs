mod days;
mod utils;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <day>");
        println!("Example: cargo run 1");
        return;
    }

    let day: u32 = args[1].parse().expect("Please provide a valid day number");

    match day {
        1 => days::day01::solve(),
        2 => days::day02::solve(),
        // Add more days here as you complete them
        // 3 => days::day03::solve(),
        _ => println!("Day {} not implemented yet!", day),
    }
}
