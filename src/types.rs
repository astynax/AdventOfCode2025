use std::process::exit;

pub trait Day {
    type Input1;
    type Input2;

    fn prepare_input1(&self) -> Result<&Self::Input1, String>;
    fn prepare_input2(&self) -> Result<&Self::Input2, String>;

    fn step1(&self, input: &Self::Input1);
    fn step2(&self, input: &Self::Input2);

    fn new() -> Self;
    fn setup(&mut self) -> Result<(), String>;
    fn run(&mut self) {
        if let Err(err) = run_result(self) {
            println!("Error: {}", err);
            exit(1);
        };
    }
}

#[inline]
fn run_result<T: Day + ?Sized>(day: &mut T) -> Result<(), String> {
    day.setup()?;
    let input1 = day.prepare_input1()?;
    day.step1(input1);
    let input2 = day.prepare_input2()?;
    day.step2(input2);
    Ok(())
}
