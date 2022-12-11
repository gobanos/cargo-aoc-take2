use colored::Colorize;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::panic::{catch_unwind, UnwindSafe};
use std::time::{Duration, Instant};

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
        generation_duration: Duration,
    },
    Success {
        solution: Box<dyn Display>,
        generation_duration: Duration,
        execution_duration: Duration,
    },
}

impl Display for RunResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RunResult::GenerationFailed { error } => {
                write!(
                    f,
                    "{}\n\t- generation error: {error:?}",
                    "GENERATION FAILED".red()
                )
            }
            RunResult::RunFailed {
                generation_duration,
                error,
            } => {
                write!(
                    f,
                    "{}\n\t- generation: {generation_duration:?}\n\t- execution error: {error:?}",
                    "EXECUTION FAILED".red()
                )
            }
            RunResult::Success {
                solution,
                generation_duration,
                execution_duration,
            } => {
                write!(
                    f,
                    "{}\n\t- generation: {generation_duration:?}\n\t- execution: {execution_duration:?}",
                    solution.to_string().green()
                )
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

#[derive(Debug)]
struct CaughtStringError {
    error: Box<str>,
}

impl Display for CaughtStringError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.error.fmt(f)
    }
}

impl Error for CaughtStringError {}

#[derive(Debug)]
struct CaughtUnknownError;

impl Display for CaughtUnknownError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for CaughtUnknownError {}

pub fn catch_unwind_or_downcast_to_error<F: FnOnce() -> R + UnwindSafe, R>(
    f: F,
) -> Result<(R, Duration), Box<dyn Error>> {
    let _ = std::io::stderr().lock();
    let start = Instant::now();
    match catch_unwind(f) {
        Ok(result) => Ok((result, start.elapsed())),
        Err(caught_panic) => match caught_panic.downcast::<String>() {
            Ok(error) => Err(Box::new(CaughtStringError {
                error: error.into_boxed_str(),
            })
            .into()),
            Err(caught_panic) => match caught_panic.downcast::<&str>() {
                Ok(error) => Err(Box::new(CaughtStringError {
                    error: error.to_string().into_boxed_str(),
                })
                .into()),
                Err(_) => Err(Box::new(CaughtUnknownError).into()),
            },
        },
    }
}
pub fn try_catch_unwind_or_downcast_to_error<
    F: FnOnce() -> Result<R, E> + UnwindSafe,
    R,
    E: Error + 'static,
>(
    f: F,
) -> Result<(R, Duration), Box<dyn Error>> {
    let _ = std::io::stderr().lock();
    let start = Instant::now();
    match catch_unwind(f) {
        Ok(Ok(result)) => Ok((result, start.elapsed())),
        Ok(Err(error)) => Err(error.into()),
        Err(caught_panic) => match caught_panic.downcast::<String>() {
            Ok(error) => Err(Box::new(CaughtStringError {
                error: error.into_boxed_str(),
            })
            .into()),
            Err(caught_panic) => match caught_panic.downcast::<&str>() {
                Ok(error) => Err(Box::new(CaughtStringError {
                    error: error.to_string().into_boxed_str(),
                })
                .into()),
                Err(_) => Err(Box::new(CaughtUnknownError).into()),
            },
        },
    }
}
