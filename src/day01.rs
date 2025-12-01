use std::fs::read_to_string;

use crate::types::{Day, StepResult};

pub struct Day01;

fn read_lines(path: &str) -> Result<Vec<String>, String> {
    let text = read_to_string(path)
        .map_err(|e| e.to_string())?;
    Ok(text
       .lines()
       .map(|l| l.to_string())
       .collect()
    )
}

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
enum Dir { L, R }

#[derive(Debug)]
struct State {
    zeroes: usize,
    position: i64,
}

fn stepper1(state: State, dir_and_count: &(Dir, usize)) -> State {
    let (dir, count) = dir_and_count;
    let pos: i64 = (match dir {
        Dir::L => state.position - (count.clone() as i64),
        Dir::R => state.position + (count.clone() as i64),
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
    if pos == 0 && current.position != 0 { zeroes += 1 }
    else if unbound_pos != pos && current.position != 0 { zeroes += 1 }
    State {
        zeroes,
        position: pos
    }
}

fn run<T>(stepper: T) -> Result<usize, String>
where T: Fn(State, &(Dir, usize)) -> State {
    let input = read_lines("input/day01.txt")?;
    let steps = input.iter()
        .map(parse).collect::<Result<Vec<(Dir, usize)>,String>>()?;
    let state = steps.iter().fold(
        State {zeroes: 0, position: 50},
        stepper
    );
    Ok(state.zeroes)
}

impl Day for Day01 {
    fn step1() -> StepResult {
        println!("Step 1: {}", run(stepper1)?);
        Ok(())
    }

    fn step2() -> StepResult {
        println!("Step 2: {}", run(stepper2)?);
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
