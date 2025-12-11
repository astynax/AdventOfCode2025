use std::collections::HashMap;

extern crate pathfinding;
use pathfinding::directed::count_paths::count_paths;

use crate::{input::read_lines, types::Day};

pub struct Input {
    network: HashMap<String, Vec<String>>,
}

impl Input {
    fn new() -> Input { Input { network: HashMap::new() } }

    fn from(lines: Vec<String>) -> Result<Input, String> {
        let mut new = Input::new();
        for line in lines {
            let mut words = line.split_whitespace();
            let from = words.next()
                .and_then(|w| w.strip_suffix(":"))
                .map(|w| Ok(w.to_owned()))
                .unwrap_or(Err(format!("Bad line: {}", line)))?;
            new.network.insert(
                from,words.map(ToOwned::to_owned).collect::<Vec<_>>()
            );
        }
        Ok(new)
    }

    fn count_paths(&self, from: &str, to: &str) -> usize {
        let empty = Vec::new();
        count_paths(
            &from.to_string(),
            |&current| self.network.get(current).unwrap_or(&empty).iter(),
            |&current| current.as_str() == to
        )
    }
}

pub struct Day11 {
    input: Input
}

impl Day for Day11 {
    type Input1 = Input;
    type Input2 = Input;

    fn get_name(&self) -> String { "Day 11".to_string() }

    fn input1(&self) -> &Self::Input1 { &self.input }
    fn input2(&self) -> &Self::Input2 { &self.input }

    fn step1(&self, input: &Self::Input1) {
        println!("Step 1: {}", input.count_paths("you", "out"));
    }

    fn step2(&self, input: &Self::Input2) {
        let s2d = input.count_paths("svr", "dac");
        let s2f = input.count_paths("svr", "fft");
        let d2f = input.count_paths("dac", "fft");
        let f2d = input.count_paths("fft", "dac");
        let d2o = input.count_paths("dac", "out");
        let f2o = input.count_paths("fft", "out");
        let result = s2d * d2f * f2o + s2f * f2d * d2o;
        println!("Step 2: {}", result);
    }

    fn new() -> Self { Day11 { input: Input::new() } }

    fn setup(&mut self) -> Result<(), String> {
        let lines = read_lines("input/day11.txt")?;
        self.input = Input::from(lines)?;
        Ok(())
    }
}
