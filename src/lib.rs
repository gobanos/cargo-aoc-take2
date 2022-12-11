use std::error::Error;
use std::fmt::{Display, Formatter};
use std::time::Duration;

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
    fn run(&self, input: &str) -> Result<RunMetrics, Box<dyn Error>>;
}

pub struct RunMetrics {
    pub result: Box<dyn Display>,
    pub generation: Duration,
    pub execution: Duration,
}

pub enum Never {}

impl Runner for Never {
    fn run(&self, _input: &str) -> Result<RunMetrics, Box<dyn Error>> {
        unimplemented!()
    }
}

impl Display for Never {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}
