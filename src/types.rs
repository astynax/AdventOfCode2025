use std::process::exit;

pub trait Day {
    type Input1;
    type Input2;

    fn input1(&self) -> &Self::Input1;
    fn input2(&self) -> &Self::Input2;

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
    day.step1(day.input1());
    day.step2(day.input2());
    Ok(())
}
