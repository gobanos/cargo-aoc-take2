use std::error::Error;
use std::fmt::{Display, Formatter};
use std::time::Duration;
use colored::Colorize;

pub mod from_input;

pub trait MaybeRunner {
    type R: Runner + Sized;

    fn into_runner(self) -> Option<Self::R>;
}

impl<T> MaybeRunner for &T {
    type R = Never;

    fn into_runner(self) -> Option<Self::R> {
        None
    }
}

pub trait Runner {
    fn run(&self, input: &str) -> RunResult;
}

pub enum RunResult {
    GenerationFailed {
        error: Box<dyn Error>,
    },
    RunFailed {
        error: Box<dyn Error>,
        generation: Duration,
    },
    Success {
        solution: Box<dyn Display>,
        generation: Duration,
        execution: Duration,
    },
}

impl Display for RunResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RunResult::GenerationFailed { error } => {
                write!(f, "{}\n\t- generation error: {error:?}", "GENERATION FAILED".red())
            }
            RunResult::RunFailed { generation, error } => {
                write!(f, "{}\n\t- generation: {generation:?}\n\t- execution error: {error:#?}", "EXECUTION FAILED".red())
            }
            RunResult::Success { solution, generation, execution } => {
                write!(f, "{}\n\t- generation: {generation:?}\n\t- execution: {execution:?}", solution.to_string().green())
            }
        }
    }
}

pub enum Never {}

impl Runner for Never {
    fn run(&self, _input: &str) -> RunResult {
        unimplemented!()
    }
}

impl Display for Never {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}
