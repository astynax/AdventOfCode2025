pub type StepResult = Result<(), String>;

pub trait Day {
    fn step1() -> StepResult;
    fn step2() -> StepResult;
}
