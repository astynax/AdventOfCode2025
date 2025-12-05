use std::ops::RangeInclusive;

use crate::types::Day;
use crate::input::read_lines;

type IdRange = RangeInclusive<usize>;

pub struct DB {
    ranges: Vec<IdRange>,
    ids: Vec<usize>,
}

impl DB {
    fn new() -> DB { DB { ranges: Vec::new(), ids: Vec::new() }}

    fn from(lines: Vec<String>) -> Result<DB, String> {
        let ranges = lines.iter().take_while(|s| !s.is_empty())
            .map(parse_range)
            .collect::<Result<Vec<_>, String>>()?;
        let ids = lines.iter()
            .skip_while(|s| !s.is_empty())
            .skip(1)
            .map(|s| parse_id(s.as_str()))
            .collect::<Result<Vec<_>, String>>()?;
        Ok(DB { ranges, ids })
    }

    fn count_fresh(&self) -> usize {
        self.ids.iter().filter(
            |id| self.ranges.iter()
                .any(|r| r.contains(id))
        ).count()
    }

    fn all_possible_fresh_ids(&self) -> usize {
        let mut v = self.ranges.clone();
        v.sort_by_key(|r| *r.start());
        match v.split_first() {
            None => {
                eprintln!("Range list shouln'd be empty!");
                0
            },
            Some((fst, rs)) => {
                let mut l = *fst.start();
                let mut r = *fst.end();
                let mut acc = 0_usize;
                for n in rs {
                    if *n.start() > r {
                        acc += r - l + 1;
                        l = *n.start();
                        r = *n.end();
                    } else {
                        r = r.max(*n.end());
                    }
                }
                acc += r - l + 1;
                acc
            },
        }
    }
}

fn parse_range(line: &String) -> Result<IdRange, String> {
    let (lhs, rhs) = line.split_once('-')
        .map(Result::Ok)
        .unwrap_or(Err(format!("Non-range: {}", line)))?;
    let l = parse_id(lhs)?;
    let r = parse_id(rhs)?;
    Ok(l..=r)
}

fn parse_id(line: &str) -> Result<usize, String> {
    line.parse::<usize>().map_err(|err| err.to_string())
}

pub struct Day05 {
    db: DB
}

impl Day for Day05 {
    type Input1 = DB;
    type Input2 = DB;

    fn get_name(&self) -> String { "Day 05".to_string() }

    fn input1(&self) -> &Self::Input1 { &self.db }
    fn input2(&self) -> &Self::Input2 { &self.db }

    fn step1(&self, input: &Self::Input1) {
        let fresh = input.count_fresh();
        println!("Step 1: {}", fresh);
    }

    fn step2(&self, input: &Self::Input2) {
        let fresh = input.all_possible_fresh_ids();
        println!("Step 2: {}", fresh);
    }

    fn new() -> Self { Day05 { db: DB::new() } }

    fn setup(&mut self) -> Result<(), String> {
        let lines = read_lines("input/day05.txt")?;
        self.db = DB::from(lines)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_possible_fresh_ids() {
        let db = DB::from(vec![
            "3-5".to_string(),
            "10-14".to_string(),
            "16-20".to_string(),
            "12-18".to_string(),
        ]).expect("Should always be Ok");
        assert_eq!(db.all_possible_fresh_ids(), 14);
    }
}
