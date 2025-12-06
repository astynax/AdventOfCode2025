use crate::types::Day;
use crate::input::{read_lines,parse_usize};

#[derive(Clone, Copy, Debug)]
pub enum Op { Mul, Add }

#[derive(Debug)]
struct Problem {
    op: Op,
    numbers: Vec<usize>,
    rows: Vec<String>,
}

pub struct Input {
    problems: Vec<Problem>,
}

impl Input {
    fn new() -> Input { Input { problems: Vec::new() } }

    fn from(lines: Vec<String>) -> Result<Input, String> {
        let last_line = lines.last().expect("Input shoudn't be empty");
        let but_last = &lines[0..lines.len() - 1];
        let ops = last_line
            .chars().enumerate()
            .filter_map(|(i, c)| match c {
                '+' => Some((i, Op::Add)),
                '*' => Some((i, Op::Mul)),
                _ => None,
            }).collect::<Vec<_>>();
        let problems = ops.iter()
            .zip(ops.iter().skip(1).map(|(i,_)| i)
                 .chain(vec![&(last_line.len() + 1)])
            ).map(|((i, op), j)| -> Result<Problem, String> {
                let rows = but_last.iter().map(
                    |s| s.get(*i..*j - 1)
                        .map(|s| Result::Ok(s.to_string()))
                        .unwrap_or(Err("oops".to_string()))
                ).collect::<Result<Vec<_>, _>>()?;
                let numbers = rows.iter()
                    .map(|s| parse_usize(s.trim()))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Problem { rows, numbers, op: *op })
            }).collect::<Result<Vec<_>, _>>()?;
        Ok(Input { problems })
    }

    fn grand_total(&self) -> usize {
        self.problems.iter()
            .map(|p| match p.op {
                Op::Add => p.numbers.iter().sum::<usize>(),
                Op::Mul => p.numbers.iter().product(),
            }).sum()
    }

    fn grand_total_rtl(&self) -> usize {
        self.problems.iter()
            .map(|p| match p.op {
                Op::Add => rtl(&p.rows).iter().sum::<usize>(),
                Op::Mul => rtl(&p.rows).iter().product(),
            }).sum()
    }
}

fn rtl(rows: &[String]) -> Vec<usize> {
    let l = rows.first().expect("Shouln't be empty").len();
    (0..l).rev()
        .map(|i| {
            let chars = rows.iter().map(
                |row| row.chars().nth(i).expect("Should present")
            );
            String::from_iter(chars)
                .trim().parse::<usize>().expect("Shoul always succeed")
        }).collect()
}

pub struct Day06 {
    input: Input
}

impl Day for Day06 {
    type Input1 = Input;
    type Input2 = Input;

    fn get_name(&self) -> String { "Day 06".to_string() }

    fn input1(&self) -> &Self::Input1 { &self.input }
    fn input2(&self) -> &Self::Input2 { &self.input }

    fn step1(&self, input: &Self::Input1) {
        println!("Step 1: {}", input.grand_total());
    }

    fn step2(&self, input: &Self::Input2) {
        println!("Step 2: {}", input.grand_total_rtl());
    }

    fn new() -> Self { Day06 { input: Input::new() } }

    fn setup(&mut self) -> Result<(), String> {
        let lines = read_lines("input/day06.txt")?;
        self.input = Input::from(lines)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Input { Input::from(vec![
        "123 328  51 64 ".to_owned(),
        " 45 64  387 23 ".to_owned(),
        "  6 98  215 314".to_owned(),
        "*   +   *   +  ".to_owned(),
    ]).expect("Should always succeed")}

    #[test]
    fn grand_total() {
        assert_eq!(example().grand_total(), 4277556);
    }

    #[test]
    fn rtl_example() {
        let ps = &example().problems;
        assert_eq!(rtl(&ps[0].rows), vec![356, 24, 1]);
        assert_eq!(rtl(&ps[3].rows), vec![4, 431, 623]);
    }

    #[test]
    fn grand_total_rtl() {
        assert_eq!(example().grand_total_rtl(), 3263827);
    }
}
