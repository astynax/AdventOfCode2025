use crate::types::Day;
use crate::input::read_single_line;

pub struct Day02 {
    input: Vec<(usize, usize)>
}

impl Day for Day02 {
    fn get_name(&self) -> String { "Day 02".to_string() }

    type Input1 = Vec<(usize, usize)>;
    type Input2 = Vec<(usize, usize)>;

    fn input1(&self) -> &Self::Input1 { &self.input }
    fn input2(&self) -> &Self::Input2 { &self.input }

    fn step1(&self, input: &Self::Input1) {
        println!("Step 1: {}", sum_matches(is_made_of_halves, input))
    }

    fn step2(&self, input: &Self::Input2) {
        println!("Step 2: {}", sum_matches(is_made_of_chunks, input))
    }

    fn new() -> Self { Day02 { input: Vec::new() } }

    fn setup(&mut self) -> Result<(), String> {
        let line = read_single_line("input/day02.txt")?;
        self.input = line
            .split(",")
            .map(parse_range)
            .collect::<Result<_, String>>()?;
        Ok(())
    }
}

fn parse_range(item: &str) -> Result<(usize, usize), String> {
    let (lhs, rhs) = item.split_once("-")
        .map(Result::Ok)
        .unwrap_or(Err(format!("Bad range: {}", item)))?;
    let l = lhs.parse::<usize>()
        .map_err(|err| err.to_string())?;
    let r = rhs.parse::<usize>()
        .map_err(|err| err.to_string())?;
    Ok((l, r))
}

fn sum_matches<T>(pred: T, ranges: &Vec<(usize, usize)>) -> usize
where T: Fn(usize) -> bool {
    let mut sum: usize = 0;
    for (f, t) in ranges {
        for number in *f..=*t {
            if pred(number) {
                sum += number;
            }
        }
    }
    sum
}

fn is_made_of_halves(number: usize) -> bool {
    let s = number.ilog10() + 1;  // a "size" of the number
    if s.rem_euclid(2) == 1 {
        return false
    }
    let n = 10_usize.pow(s / 2);
    let l = number.div_euclid(n);
    let r = number.rem_euclid(n);
    l == r
}

fn is_made_of_chunks(number: usize) -> bool {
    let s = number.to_string();
    let l = s.len();
    let h = l / 2;
    for i in 1..=h {
        if l.rem_euclid(i) != 0 { continue }
        let n = l.div_euclid(i);
        let s2 = s.get(0..i).unwrap_or("").repeat(n);
         if s2 == s { return true }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_on_examples() {
        assert!(is_valid_id(101));
        assert!(!is_valid_id(1010));
        assert!(!is_valid_id(1188511885));
        assert!(is_valid_id(1188511886));
    }

    #[test]
    fn is_made_of_chunks_examples() {
        assert!(is_made_of_chunks(101101));
        assert!(!is_made_of_chunks(101111));
        assert!(is_made_of_chunks(123123123));
        assert!(is_made_of_chunks(11111111));
    }
}
