use crate::types::Day;
use crate::input::{parse_each_with, read_lines};

type Step = (Dir, usize);

pub struct Day01 { lines: Vec<Step> }

fn parse(l: &String) -> Result<(Dir, usize), String> {
    let dir: Dir = l.get(0..1)
        .and_then(|s| match s {
            "L" => Some(Dir::L),
            "R" => Some(Dir::R),
            _ => None
        })
        .map(Result::Ok)
        .unwrap_or(Err(format!("Bad prefix: {}", l).to_string()))?;
    let count: usize = l.get(1..)
        .map(|s| if s.is_empty() {
            Ok(1)
        } else {
            s.parse::<usize>()
                .map_err(|err| err.to_string())
        })
        .unwrap_or(Err(format!("Bad suffix: {}", l).to_string()))?;
    Ok((dir, count))
}

#[derive(Debug)]
pub enum Dir { L, R }

#[derive(Debug)]
struct State {
    zeroes: usize,
    position: i64,
}

fn stepper1(state: State, dir_and_count: &(Dir, usize)) -> State {
    let (dir, count) = dir_and_count;
    let pos: i64 = (match dir {
        Dir::L => state.position - (*count as i64),
        Dir::R => state.position + (*count as i64),
    }).rem_euclid(100);
    let zeroes = if pos == 0 { state.zeroes + 1 } else { state.zeroes };
    State {
        zeroes,
        position: pos
    }
}

fn stepper2(current: State, dir_and_count: &(Dir, usize)) -> State {
    let (dir, raw_count) = dir_and_count;
    let count = raw_count.rem_euclid(100) as i64;
    let full_rotations = raw_count.div_euclid(100);
    let unbound_pos: i64 = match dir {
        Dir::L => current.position - count,
        Dir::R => current.position + count,
    };
    let pos = unbound_pos.rem_euclid(100);
    let mut zeroes = current.zeroes;
    zeroes += full_rotations;
    if current.position != 0 && (
       pos == 0 || unbound_pos != pos
    ) { zeroes += 1 }
    State {
        zeroes,
        position: pos
    }
}

fn run<T>(input: &[Step], stepper: T) -> usize
where T: Fn(State, &Step) -> State {
    input.iter().fold(
        State {zeroes: 0, position: 50},
        stepper
    ).zeroes
}

impl Day for Day01 {
    type Input1 = Vec<(Dir, usize)>;
    type Input2 = Vec<(Dir, usize)>;

    fn prepare_input1(&self) -> Result<&Self::Input1, String> {
        Ok(&self.lines)
    }

    fn prepare_input2(&self) -> Result<&Self::Input2, String> {
        Ok(&self.lines)
    }

    fn step1(&self, input: &Self::Input1) {
        println!("Step 1: {}", run(input, stepper1));
    }

    fn step2(&self, input: &Self::Input2) {
        println!("Step 2: {}", run(input, stepper2));
    }

    fn new() -> Self { Day01 { lines: Vec::new() } }

    fn setup(&mut self) -> Result<(), String> {
        let input = read_lines("input/day01.txt")?;
        self.lines = parse_each_with(parse, input)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn step<T>(stepper: T, pos: i64, dir: Dir, count: usize) -> usize
    where T: Fn(State, &(Dir, usize)) -> State {
        stepper(State { position: pos, zeroes: 0}, &(dir, count)).zeroes
    }

    #[test]
    fn test_small_turn() {
        assert_eq!(step(stepper2, 5, Dir::L, 10), 1);
        assert_eq!(step(stepper2, 5, Dir::R, 10), 0);
        assert_eq!(step(stepper2, 95, Dir::R, 10), 1);
    }

    #[test]
    fn test_small_turn_from_zero() {
        assert_eq!(step(stepper2, 0, Dir::L, 5), 0);
        assert_eq!(step(stepper2, 0, Dir::R, 5), 0);
    }

    #[test]
    fn test_one_full_turn() {
        assert_eq!(step(stepper2, 5, Dir::L, 100), 1);
        assert_eq!(step(stepper2, 5, Dir::R, 100), 1);
    }

    #[test]
    fn test_two_and_a_half_turns() {
        assert_eq!(step(stepper2, 5, Dir::R, 250), 2);
        assert_eq!(step(stepper2, 5, Dir::L, 250), 3);
    }

    #[test]
    fn test_full_turns_from_zero_to_zero() {
        assert_eq!(step(stepper2, 0, Dir::R, 500), 5);
        assert_eq!(step(stepper2, 0, Dir::L, 500), 5);
    }

    #[test]
    fn test_big_turns_to_zero() {
        assert_eq!(step(stepper2, 5, Dir::R, 495), 5);
        assert_eq!(step(stepper2, 5, Dir::L, 505), 6);
    }
}
