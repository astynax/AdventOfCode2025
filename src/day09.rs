#![allow(dead_code, reason = "Draft")]
#![allow(clippy::comparison_chain, reason = "Draft")]
#![allow(clippy::map_entry, reason = "Draft")]
#![allow(clippy::print_literal, reason = "Draft")]

use std::collections::HashMap;

use crate::{input::{parse_usize, read_lines}, types::Day};

type Pos = (usize, usize);

pub struct Input {
    tiles: Vec<Pos>
}

impl Input {
    fn new() -> Input { Input { tiles: Vec::new() } }

    fn from(lines: Vec<String>) -> Result<Input, String> {
        let tiles = lines.iter().map(|line| -> Result<Pos, String> {
            let (rx, ry) = line.split_once(",")
                .map(Result::Ok)
                .unwrap_or(Err(format!("Comma expected: {}", line)))?;
            let x = parse_usize(rx)?;
            let y = parse_usize(ry)?;
            Ok((x, y))
        }).collect::<Result<Vec<_>, _>>()?;
        Ok(Input { tiles })
    }

    fn largest_rectangle(&self) -> usize {
        let mut max = 0_usize;
        for (i, p1) in self.tiles.iter().enumerate() {
            for p2 in self.tiles.iter().skip(i + 1) {
                let m = Rect::from(p1, p2).area();
                if m > max { max = m }
            }
        };
        max
    }
}

struct Rect {
    x1: usize, y1: usize,
    x2: usize, y2: usize,
}

impl Rect {
    fn from((p1x, p1y): &Pos, (p2x, p2y): &Pos) -> Rect {
        let mut x1 = *p1x;
        let mut x2 = *p2x;
        if x1 > x2 { let t = x1; x1 = x2; x2 = t };
        let mut y1 = *p1y;
        let mut y2 = *p2y;
        if y1 > y2 { let t = y1; y1 = y2; y2 = t };
        Rect { x1, y1, x2, y2 }
    }

    fn contains(&self, (px, py): &Pos) -> bool {
        *px > self.x1 && *px < self.x2 && *py > self.y1 && *py < self.y2
    }

    fn area(&self) -> usize {
        (1 + self.x2 - self.x1) * (1 + self.y2 - self.y1)
    }
}

#[derive(Debug,PartialEq)]
enum Dir { U, D, L, R }

struct Path {
    steps: Vec<(Pos, Pos, Dir)>
}

impl Path {
    fn from(points: Vec<Pos>) -> Path {
        let top = points.iter()
            .enumerate().min_by_key(|(_, (_, y))| *y)
            .expect("Shoudn't be empty").0;
        let shifted = points.iter()
            .skip(top).chain(points.iter().take(top));
        let first = *shifted.clone().next().expect("Should present");
        let pairs = shifted.clone().zip(
            shifted.skip(1).chain(vec![&first])
        );
        let steps = pairs.map(|(p1, p2)| {
            let (x1, y1) = *p1;
            let (x2, y2) = *p2;
            let d: Dir = (if x1 == x2 {
                if y1 < y2 { Some(Dir::D) }
                else if y1 > y2 { Some(Dir::U) }
                else { None }
            } else if y1 == y2 {
                if x1 < x2 { Some(Dir::R) }
                else if x1 > x2 { Some(Dir::L) }
                else { None }
            } else { None }
            ).expect("Bad segment");
            (*p1, *p2, d)
        }).collect::<Vec<_>>();
        Path { steps }
    }
}

struct Scale {
    down: HashMap<usize, usize>,
    up: HashMap<usize, usize>,
}

impl Scale {
    fn from<'a>(values: impl Iterator<Item = &'a usize>) -> Scale {
        let mut vs = values.map(|x| x.to_owned()).collect::<Vec<_>>();
        vs.sort();
        let mut down = HashMap::new();
        let mut up = HashMap::new();
        let mut i = 0_usize;
        for v in vs {
            if !down.contains_key(&v) {
                down.insert(v, i);
                up.insert(i, v);
                i += 2;
            }
        }
        Scale { down, up }
    }

    fn down(&self, v: usize) -> usize {
        *self.down.get(&v).expect("Shoult be scaled")
    }

    fn up(&self, v: usize) -> usize {
        *self.up.get(&v).expect("Shoult be scaled")
    }
}

pub struct Day09 {
    input: Input
}

impl Day for Day09 {
    type Input1 = Input;
    type Input2 = Input;

    fn get_name(&self) -> String { "Day 09".to_string() }

    fn input1(&self) -> &Self::Input1 { &self.input }
    fn input2(&self) -> &Self::Input2 { &self.input }

    fn step1(&self, input: &Self::Input1) {
        let result = input.largest_rectangle();
        println!("Step 1: {}", result);
    }

    fn step2(&self, input: &Self::Input2) {
        let mut v = input.tiles.iter().map(|(x, _)| x).collect::<Vec<_>>();
        println!("{}", v.len());
        v = input.tiles.iter().map(|(_, y)| y).collect::<Vec<_>>();
        println!("{}", v.len());
        println!("Step 2: {}", "TODO");
    }

    fn new() -> Self { Day09 { input: Input::new() } }

    fn setup(&mut self) -> Result<(), String> {
        let lines = read_lines("input/day09.txt")?;
        self.input = Input::from(lines)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_from() {
        let p = Path::from(vec![
            (5, 5), (5, 1), (8, 1), (8, 8), (1, 8), (1, 5)
        ]);
        assert_eq!(p.steps, vec![
            ((5, 1), (8, 1), Dir::R),
            ((8, 1), (8, 8), Dir::D),
            ((8, 8), (1, 8), Dir::L),
            ((1, 8), (1, 5), Dir::U),
            ((1, 5), (5, 5), Dir::R),
            ((5, 5), (5, 1), Dir::U),
        ])
    }

    #[test]
    fn scaling() {
        let s = Scale::from([200, 1_usize, 5, 200, 1000].iter());
        assert_eq!(s.down(200), 4);
        assert_eq!(s.up(6), 1000);
        assert_eq!(s.up(s.down(200)), 200);
    }
}
