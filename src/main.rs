mod types;
mod input;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

use types::Day;

fn main() {
    let args: Vec<usize> = std::env::args().skip(1).map(
        |s| s.parse::<usize>()
            .unwrap_or_else(|err| {
                eprintln!("{}", err);
                std::process::exit(1);
            })).collect();
    let days = if args.is_empty() { (1..12).collect() } else { args };
    if days.contains(&1) { day01::Day01::new().run(); }
    if days.contains(&2) { day02::Day02::new().run(); }
    if days.contains(&3) { day03::Day03::new().run(); }
    if days.contains(&4) { day04::Day04::new().run(); }
    if days.contains(&5) { day05::Day05::new().run(); }
    if days.contains(&6) { day06::Day06::new().run(); }
    if days.contains(&7) { day07::Day07::new().run(); }
}
