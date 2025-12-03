mod types;
mod input;
mod day01;
mod day02;
mod day03;

use types::Day;

use std;

fn main() {
    let args: Vec<usize> = std::env::args().skip(1).map(
        |s| s.parse::<usize>()
            .unwrap_or_else(|err| {
                eprintln!("{}", err.to_string());
                std::process::exit(1);
            })).collect();
    if args.is_empty() || args.contains(&1) { day01::Day01::new().run(); }
    if args.is_empty() || args.contains(&2) { day02::Day02::new().run(); }
    if args.is_empty() || args.contains(&3) { day03::Day03::new().run(); }
}
