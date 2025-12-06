define(`DayXX', format(`Day%s', XX))dnl
use crate::types::Day;

pub struct DayXX {
    input: ()
}

impl Day for DayXX {
    type Input1 = ();
    type Input2 = ();

    fn get_name(&self) -> String { "Day XX".to_string() }

    fn input1(&self) -> &Self::Input1 { &self.input }
    fn input2(&self) -> &Self::Input2 { &self.input }

    fn step1(&self, input: &Self::Input1) {
        println!("Step 1: {:?}", input);
    }

    fn step2(&self, input: &Self::Input2) {
        println!("Step 2: {:?}", input);
    }

    fn new() -> Self { DayXX { input: () } }

    fn setup(&mut self) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
