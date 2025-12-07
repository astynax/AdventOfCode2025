use std::collections::{HashMap, HashSet};

use crate::{input::read_lines, types::Day};

type Pos = (usize, usize);

#[derive(Debug)]
pub struct Input {
    height: usize,
    start: Pos,
    splitters: HashSet<Pos>,
}

impl Input {
    fn new() -> Input { Input {
        height: 0, start: (0, 0), splitters: HashSet::new()
    }}

    fn from(lines: Vec<String>) -> Result<Input, String> {
        let start_x = lines
            .first().expect("Should be at least one line")
            .find('S').expect("Start should be on the first line");
        let start = (start_x, 0_usize);
        let height = lines.len();
        let splitters = lines.iter().enumerate().skip(1).flat_map(
            |(y, line)| line.chars().enumerate().filter_map(
                move |(x, c)| if c == '^' { Some((x, y)) } else { None }
            )
        ).collect::<HashSet<_>>();
        Ok(Input { height, start, splitters })
    }

    fn touched_splitters(&self) -> usize {
        let mut rays: HashSet<usize> = HashSet::new();
        rays.insert(self.start.0);
        let mut y = 1_usize;
        let mut splits = 0_usize;
        while y < self.height {
            y += 1;
            let mut new: HashSet<usize> = HashSet::new();
            for ray in rays {
                if self.splitters.contains(&(ray, y)) {
                    splits += 1;
                    new.insert(ray.wrapping_sub(1));
                    new.insert(ray + 1);
                } else {
                    new.insert(ray);
                }
            }
            rays = new;
        }
        splits
    }

    fn timelines(&self) -> usize {
        let mut rays: HashMap<usize, usize> = HashMap::new();
        rays.insert(self.start.0, 1);
        let mut y = 1_usize;
        while y < self.height {
            y += 1;
            let mut new: HashMap<usize, usize> = HashMap::new();
            for (ray, lines) in rays {
                if self.splitters.contains(&(ray, y)) {
                    addsert(&mut new, ray.wrapping_sub(1), lines);
                    addsert(&mut new, ray.wrapping_add(1), lines);
                } else {
                    addsert(&mut new, ray, lines);
                }
            }
            rays = new;
        }
        rays.values().sum()
    }
}

#[inline]
fn addsert(map: &mut HashMap<usize, usize>, k: usize, v: usize) {
    let current = *map.get(&k).unwrap_or(&0);
    map.insert(k, v + current);
}

pub struct Day07 {
    input: Input
}

impl Day for Day07 {
    type Input1 = Input;
    type Input2 = Input;

    fn get_name(&self) -> String { "Day 07".to_string() }

    fn input1(&self) -> &Self::Input1 { &self.input }
    fn input2(&self) -> &Self::Input2 { &self.input }

    fn step1(&self, input: &Self::Input1) {
        println!("Step 1: {}", input.touched_splitters());
    }

    fn step2(&self, input: &Self::Input2) {
        println!("Step 2: {}", input.timelines());
    }

    fn new() -> Self { Day07 { input: Input::new() } }

    fn setup(&mut self) -> Result<(), String> {
        let lines = read_lines("input/day07.txt")?;
        self.input = Input::from(lines)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn touched_splitters() {
        let i = Input::from(vec![
            "....S....".to_string(),
            ".........".to_string(),
            "....^....".to_string(),
            ".........".to_string(),
            "...^.....".to_string(),
            ".........".to_string(),
            "..^.^.^..".to_string(),
            ".........".to_string(),
        ]).expect("Should always work");
        assert_eq!(i.touched_splitters(), 4);
    }

    #[test]
    fn timelines() {
        let i = Input::from(vec![
            "....S....".to_string(),
            "....1....".to_string(),
            "....^....".to_string(),
            "...1.1...".to_string(),
            "...^.^...".to_string(),
            "..1.2.1..".to_string(),
            "..^.^....".to_string(),
            ".1.3.21..".to_string(),
        ]).expect("Should always work");
        assert_eq!(i.timelines(), 7);
    }
}
