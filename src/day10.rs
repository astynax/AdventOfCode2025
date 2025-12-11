use crate::input::read_lines;
use crate::types::Day;

extern crate combine;
use combine::{between, eof, many1, one_of, sep_by1, EasyParser, Parser};
use combine::parser::char::{spaces,digit,char};

extern crate pathfinding;
use pathfinding::directed::bfs::bfs;

struct Machine {
    target: u16,
    buttons: Vec<Vec<u16>>,
    joltages: Vec<u16>,
}

impl Machine {
    fn from(line: &str) -> Result<Machine, String> {
        let number = many1(digit()).and_then(|s: String| s.parse::<u16>());
        let numbers = sep_by1::<Vec<u16>, _, _, _>(number, char(','));
        let led_p = one_of(".#".chars());
        let target_p = between(char('['), char(']'), many1(led_p).map(leds_to_u16));
        let button_p = between(char('('), char(')'), numbers.clone());
        let joltages_p = between( char('{'), char('}'), numbers);
        let mut line_p = (
            target_p,
            spaces(),
            many1(button_p.skip(spaces())),
            spaces(),
            joltages_p,
            eof()
        ).map(|(target, _, buttons, _, joltages, _)|
              Machine { target, buttons, joltages }
        );
        line_p.easy_parse(line)
            .map(|(m, _)| m)
            .map_err(|e| e.to_string())
    }

    fn fewest_presses_to_init(&self) -> usize {
        let bs = self.buttons.iter().map(button_to_bits).collect::<Vec<u16>>();
        let search = bfs(&0_u16, |&v| {
            bs.iter().map(|b| v ^ *b).collect::<Vec<_>>()
        }, |s| { *s == self.target }
        ).expect("Should be always possible");
        search.len() - 1 // without the start point
    }

    fn fewest_presses_to_power(&self) -> usize {
        println!("{:?}", self.joltages);
        let mut start: Vec<u16> = Vec::new();
        (0..self.joltages.len()).for_each(|_| start.push(0_u16));
        let search = bfs(&start, |current| {
            self.buttons.iter().filter_map(
                |button| press_button(&self.joltages, current, button)
            ).collect::<Vec<_>>()
        }, |s| { *s == self.joltages }
        ).expect("Should be always possible");
        search.len() - 1 // without the start point
    }
}

#[inline]
fn leds_to_u16(leds: Vec<char>) -> u16 {
    leds.iter().rfold(0, |acc, c| acc * 2 + if *c == '#' { 1 } else { 0 })
}

#[inline]
#[allow(clippy::ptr_arg)]
fn button_to_bits(indices: &Vec<u16>) -> u16 {
    indices.iter().map(|i| 2_u16.pow((*i).into())).sum()
}

#[allow(clippy::ptr_arg)]
fn press_button(
    max: &Vec<u16>, current: &Vec<u16>, button: &Vec<u16>
) -> Option<Vec<u16>> {
    let mut out = current.clone();
    for pos in button {
        let i = *pos as usize;
        if out[i] >= max[i] {
            return None
        }
        out[i] += 1;
    }
    Some(out)
}

pub struct Input {
    machines: Vec<Machine>
}

impl Input {
    fn new() -> Input { Input { machines: Vec::new() } }

    fn from(lines: Vec<String>) -> Result<Input, String> {
        let machines = lines.iter()
            .map(|line| Machine::from(line))
            .collect::<Result<_, _>>()?;
        Ok(Input { machines })
    }

    fn fewest_presses_to_init(&self) -> usize {
        self.machines.iter().map(|m| m.fewest_presses_to_init()).sum()
    }

    fn fewest_presses_to_power(&self) -> usize {
        self.machines.iter().map(|m| m.fewest_presses_to_power()).sum()
    }
}

pub struct Day10 {
    input: Input
}

impl Day for Day10 {
    type Input1 = Input;
    type Input2 = Input;

    fn get_name(&self) -> String { "Day 10".to_string() }

    fn input1(&self) -> &Self::Input1 { &self.input }
    fn input2(&self) -> &Self::Input2 { &self.input }

    fn step1(&self, input: &Self::Input1) {
        println!("Step 1: {}", input.fewest_presses_to_init());
    }

    fn step2(&self, input: &Self::Input2) {
        println!("Step 2: {}", input.fewest_presses_to_power());
    }

    fn new() -> Self { Day10 { input: Input::new() } }

    fn setup(&mut self) -> Result<(), String> {
        let lines = read_lines("input/day10_example.txt")?;
        self.input = Input::from(lines)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn machine_from() {
        let m = Machine::from("[#.#.] (0,1) (1,2) {4,5}")
            .expect("Should be parsable");
        assert_eq!(m.target, 5);
        assert_eq!(m.buttons, vec![vec![0, 1], vec![1, 2]]);
        assert_eq!(m.joltages, vec![4, 5]);
    }
}
