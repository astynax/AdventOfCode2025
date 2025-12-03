use crate::{input::read_lines, types::Day};

type Bank = Vec<u8>;

pub struct Day03 {
    banks: Vec<Bank>
}

impl Day for Day03 {
    type Input1 = Vec<Bank>;
    type Input2 = Vec<Bank>;

    fn get_name(&self) -> String { "Day 03".to_string() }

    fn input1(&self) -> &Self::Input1 { &self.banks }
    fn input2(&self) -> &Self::Input2 { &self.banks }

    fn step1(&self, input: &Self::Input1) {
        let sum: usize = input.iter()
            .map(|b| max_power(b, 2))
            .sum();
        println!("Step 1: {}", sum)
    }

    fn step2(&self, input: &Self::Input2) {
        let sum: usize = input.iter()
            .map(|b| max_power(b, 12))
            .sum();
        println!("Step 1: {}", sum)
    }

    fn new() -> Self { Day03 { banks: Vec::new() } }

    fn setup(&mut self) -> Result<(), String> {
        let lines = read_lines("input/day03.txt")?;
        self.banks = lines.iter()
            .map(|s| parse_bank(s))
            .collect::<Result<_, _>>()?;
        Ok(())
    }
}

fn parse_bank(line: &str) -> Result<Bank, String> {
    line.chars().map(|c| {
        let digit = c.to_digit(10)
            .map(Result::Ok)
            .unwrap_or(Err(format!("Non digit: {}", c)))?;
        if digit > 9 { return Err("impossible!".to_string()) }
        Ok(digit as u8)
    }).collect()
}

fn max_power(bank: &Bank, of: usize) -> usize {
    let mut acc = 0;
    let end = bank.len() - 1;
    let mut stop = bank.len() - of;
    let mut start = 0_usize;
    let mut max = 0_u8;
    while stop <= end {
        for (i, x) in bank
            .iter().enumerate().take(stop + 1).skip(start)  // start..=stop
        {
            if max < *x {
                max = *x;
                start = i + 1;
            }
        }
        acc = 10 * acc + max as usize;
        max = 0;
        stop += 1;
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_power_of_2() {
        fn pwr(line: &str) -> usize {
            max_power(&parse_bank(&line.to_string()).unwrap(), 2)
        }
        assert_eq!(pwr("987654321111111"), 98);
        assert_eq!(pwr("811111111111119"), 89);
        assert_eq!(pwr("234234234234278"), 78);
        assert_eq!(pwr("818181911112111"), 92);
    }

    #[test]
    fn max_power_of_12() {
        fn pwr(line: &str) -> usize {
            max_power(&parse_bank(&line.to_string()).unwrap(), 12)
        }
        assert_eq!(pwr("987654321111111"), 987654321111);
        assert_eq!(pwr("811111111111119"), 811111111119);
        assert_eq!(pwr("234234234234278"), 434234234278);
        assert_eq!(pwr("818181911112111"), 888911112111);
    }
}
