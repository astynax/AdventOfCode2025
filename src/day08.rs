use std::collections::{HashMap, HashSet};

use crate::types::Day;
use crate::input::{parse_usize,read_lines};

type Jbox = (usize, usize, usize);

pub struct Input {
    boxes: Vec<Jbox>,
}

impl Input {
    fn new() -> Input { Input { boxes: Vec::new() } }

    fn from(lines: Vec<String>) -> Result<Input, String> {
        let boxes: Vec<Jbox> = lines.iter().map(|s| {
            match s.split(",").collect::<Vec<_>>().as_slice() {
                [rx, ry, rz] => {
                    let x = parse_usize(rx)?;
                    let y = parse_usize(ry)?;
                    let z = parse_usize(rz)?;
                    Ok((x, y, z))
                }
                _ => Err(format!("Should be exactly 3 items: {}", s)),
            }
        }).collect::<Result<Vec<_>, _>>()?;
        Ok(Input { boxes })
    }

    fn sorted_deltas(&self) -> Vec<(usize, usize, usize)> {
        let mut result: Vec<(usize, usize, usize)> = Vec::new();
        for (i, first) in self.boxes.iter().enumerate() {
            for (j, second) in self.boxes.iter().enumerate().skip(i + 1) {
                result.push((distance(first, second), i, j));
            }
        }
        result.sort_by_key(|(distance, _, _)| *distance);
        result
    }

    fn circuits_after_joins(&self, limit: usize) -> usize {
        let deltas = self.sorted_deltas();
        let mut circuits: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut jbox2circuit: HashMap<usize, usize> = HashMap::new();
        for i in 0..self.boxes.len() {
            let mut s = HashSet::new();
            s.insert(i);
            circuits.insert(i, s);
            jbox2circuit.insert(i, i);
        }
        let source = deltas.iter()
            .take(if limit > 0 { limit } else { deltas.len() });
        for (_, a, b) in source {
            let ca = *jbox2circuit.get(a).expect("Impossible");
            let cb = *jbox2circuit.get(b).expect("Impossible");
            if ca != cb {
                let sb = circuits.get(&cb).expect("Impossible").clone();
                let sa = circuits.get_mut(&ca).expect("Impossible");
                for x in sb {
                    sa.insert(x);
                    jbox2circuit.insert(x, ca);
                }
                circuits.remove(&cb);
                if circuits.len() == 1 {
                    let ba = self.boxes.get(*a)
                        .expect("Should present");
                    let bb = self.boxes.get(*b)
                        .expect("Should present");
                    return ba.0 * bb.0
                }
            }
        }
        let mut sizes = circuits.values().map(|s| s.len()).collect::<Vec<_>>();
        sizes.sort_by_key(|x| -(*x as isize));
        sizes.iter().take(3).product()
    }
}

fn distance(first: &Jbox, second: &Jbox) -> usize {
    let (x1, y1, z1) = first;
    let (x2, y2, z2) = second;
    let dx = square_of_difference(x1, x2);
    let dy = square_of_difference(y1, y2);
    let dz = square_of_difference(z1 ,z2);
    (dx + dy + dz).isqrt()
}

#[inline]
fn square_of_difference(a: &usize, b: &usize) -> usize {
    (*a as isize - *b as isize).pow(2) as usize
}

pub struct Day08 {
    input: Input
}

impl Day for Day08 {
    type Input1 = Input;
    type Input2 = Input;

    fn get_name(&self) -> String { "Day 08".to_string() }

    fn input1(&self) -> &Self::Input1 { &self.input }
    fn input2(&self) -> &Self::Input2 { &self.input }

    fn step1(&self, input: &Self::Input1) {
        let result = input.circuits_after_joins(1000);
        println!("Step 1: {}", result);
    }

    fn step2(&self, input: &Self::Input2) {
        let result = input.circuits_after_joins(0);
        println!("Step 2: {}", result);
    }

    fn new() -> Self { Day08 { input: Input::new() } }

    fn setup(&mut self) -> Result<(), String> {
        let lines = read_lines("input/day08.txt")?;
        self.input = Input::from(lines)?;
        Ok(())
    }
}
