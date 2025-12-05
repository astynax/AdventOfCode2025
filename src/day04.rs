use std::collections::HashSet;

use crate::types::Day;
use crate::input::read_lines;

type Pos = (usize, usize);

#[derive(Clone)]
pub struct Grid {
    cells: HashSet<Pos>
}

pub struct Day04 {
    grid: Grid
}

impl Day for Day04 {
    type Input1 = Grid;
    type Input2 = Grid;

    fn get_name(&self) -> String { "Day 04".to_string() }

    fn input1(&self) -> &Self::Input1 { &self.grid }
    fn input2(&self) -> &Self::Input2 { &self.grid }

    fn step1(&self, input: &Self::Input1) {
        let accessable = input.accessible().count();
        println!("Step 1: {}", accessable);
    }

    fn step2(&self, input: &Self::Input2) {
        let remaining = input.cells.len() - input.clone().clean().cells.len();
        println!("Step 2: {}", remaining);
    }

    fn new() -> Self { Day04 { grid: Grid::new() } }

    fn setup(&mut self) -> Result<(), String> {
        let lines = read_lines("input/day04.txt")?;
        self.grid = Grid::from(lines);
        Ok(())
    }
}

const OFFSETS: &[(isize, isize)] = &[
    (-1, -1), (0, -1), (1, -1),
    (-1, 0),           (1, 0),
    (-1, 1),  (0, 1),  (1, 1),
];

impl Grid {
    fn new() -> Grid { Grid { cells: HashSet::new() } }

    fn from(lines: Vec<String>) -> Grid {
        let mut cells = HashSet::new();
        lines.iter().enumerate().for_each(|(y, row)| {
            row.chars().enumerate().for_each(|(x, c)| {
                if c == '@' { cells.insert((x, y)); };
            });
        });
        Grid { cells }
    }

    fn neibs(&self, (x, y): Pos) -> usize {
        OFFSETS.iter().filter_map(
            |(dx, dy)|
            x.checked_add_signed(*dx).and_then(
                |nx|
                y.checked_add_signed(*dy).map(
                    |ny| (nx, ny))))
            .filter(|pos| self.cells.contains(pos))
            .count()
    }

    fn accessible(&self) -> impl Iterator<Item = Pos> {
        self.cells.iter()
            .map(|pos| (pos, self.neibs(*pos)))
            .filter(|(_, neibs)| *neibs < 4)
            .map(|(pos, _)| *pos)
    }

    fn clean(&mut self) -> &Self {
        loop {
            let ps = self.accessible().collect::<Vec<_>>();
            if ps.is_empty() { break };
            ps.iter().for_each(|p| { self.cells.remove(p); });
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_from_and_neibs() {
        let grid = Grid::from(vec![
            "@..".to_string(),
            ".@.".to_string(),
            "@@@".to_string(),
        ]);
        assert_eq!(grid.neibs((0, 0)), 1);
        assert_eq!(grid.neibs((1, 1)), 4);
        assert_eq!(grid.neibs((1, 2)), 3);
        assert_eq!(grid.neibs((2, 2)), 2);
    }
}
