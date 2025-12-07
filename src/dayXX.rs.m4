define(`DayXX', format(`Day%s', XX))dnl
use crate::types::Day;

pub struct Input {}

impl Input {
    fn new() -> Input { Input {} }
}

pub struct DayXX {
    input: Input
}

impl Day for DayXX {
    type Input1 = Input;
    type Input2 = Input;

    fn get_name(&self) -> String { "Day XX".to_string() }

    fn input1(&self) -> &Self::Input1 { &self.input }
    fn input2(&self) -> &Self::Input2 { &self.input }

    fn step1(&self, _input: &Self::Input1) {
        println!("Step 1: {}", "TODO");
    }

    fn step2(&self, _input: &Self::Input2) {
        println!("Step 2: {}", "TODO");
    }

    fn new() -> Self { DayXX { input: Input::new() } }

    fn setup(&mut self) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
