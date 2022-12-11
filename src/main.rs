/// GENERATED BY CARGO-AOC
/// Those types are used by cargo-aoc to expose the various user-written functions
/// and provide the correct input
pub mod __aoc {
    use std::fmt::{Display, Formatter};

    pub struct Aoc<Day, Part, Version> {
        day: Day,
        part: Part,
        version: Version,
    }

    impl<Day, Part, Version> Aoc<Day, Part, Version> {
        pub fn new(day: Day, part: Part, version: Version) -> Self {
            Aoc { day, part, version }
        }

        pub fn day(&self) -> &Day {
            &self.day
        }

        pub fn day_mut(&mut self) -> &mut Day {
            &mut self.day
        }

        pub fn part(&self) -> &Part {
            &self.part
        }

        pub fn part_mut(&mut self) -> &mut Part {
            &mut self.part
        }

        pub fn version(&self) -> &Version {
            &self.version
        }

        pub fn version_mut(&mut self) -> &mut Version {
            &mut self.version
        }
    }

    impl<Day, Part, Version> Display for Aoc<Day, Part, Version>
    where
        Day: Display,
        Part: Display,
        Version: Display,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let Self { day, part, version } = self;
            write!(f, "{day} - {part} - {version}")
        }
    }

    impl<Day, Part> Display for Aoc<Day, Part, Base>
    where
        Day: Display,
        Part: Display,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let Self {
                day,
                part,
                version: _,
            } = self;
            write!(f, "{day} - {part}")
        }
    }

    pub struct Day1;

    impl Display for Day1 {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Day 1")
        }
    }

    pub struct Part1;

    impl Display for Part1 {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Part 1")
        }
    }

    pub struct Part2;

    impl Display for Part2 {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Part 2")
        }
    }

    pub struct Base;

    #[derive(Default)]
    pub struct Alt1 {
        name: Option<&'static str>,
    }

    impl Alt1 {
        pub fn set_name(&mut self, name: &'static str) {
            self.name = Some(name);
        }
    }

    impl Display for Alt1 {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            if let Some(name) = self.name {
                write!(f, "Alt 1 ({name})")
            } else {
                write!(f, "Alt 1")
            }
        }
    }

    #[derive(Default)]
    pub struct Alt2 {
        name: Option<&'static str>,
    }

    impl Alt2 {
        pub fn set_name(&mut self, name: &'static str) {
            self.name = Some(name);
        }
    }

    impl Display for Alt2 {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            if let Some(name) = self.name {
                write!(f, "Alt 2 ({name})")
            } else {
                write!(f, "Alt 2")
            }
        }
    }
}

/// GENERATED BY CARGO-AOC
///  Call each combination of day, part and version (maybe add years too?) with the correct input.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use __aoc::*;
    use cargo_aoc_take2::{MaybeRunner, Runner};

    let input = include_str!("../day1.txt");

    if let Some(runner) = Aoc::new(Day1, Part1, Base).into_runner() {
        let result = runner.run(input);
        println!(
            "[{runner}]: {result}\n"
        );
    }

    if let Some(runner) = Aoc::new(Day1, Part1, Alt1::default()).into_runner() {
        let result = runner.run(input);
        println!(
            "[{runner}]: {result}\n"
        );
    }

    if let Some(runner) = Aoc::new(Day1, Part1, Alt2::default()).into_runner() {
        let result = runner.run(input);
        println!(
            "[{runner}]: {result}\n"
        );
    }

    if let Some(runner) = Aoc::new(Day1, Part2, Base).into_runner() {
        let result = runner.run(input);
        println!(
            "[{runner}]: {result}\n"
        );
    }

    if let Some(runner) = Aoc::new(Day1, Part2, Alt1::default()).into_runner() {
        let result = runner.run(input);
        println!(
            "[{runner}]: {result}\n"
        );
    }

    if let Some(runner) = Aoc::new(Day1, Part2, Alt2::default()).into_runner() {
        let result = runner.run(input);
        println!(
            "[{runner}]: {result}\n"
        );
    }

    Ok(())
}

mod day1 {
    use fnv::FnvHashSet;
    use std::collections::HashSet;
    use std::num::ParseIntError;
    // Likely to be put in a `aoc-runner-extra` crate
    use cargo_aoc_take2::from_input::{LinesIter, LinesVec, Parse};

    // Based on : #[aoc(day1, part1)]
    fn part1(LinesVec(freqs): &LinesVec<Parse<i32>>) -> i32 {
        freqs.iter().map(|Parse(n)| n).sum()
    }

    /// GENERATED BY CARGO-AOC
    /// Replace the default `Runner` implementation by the custom one, see below
    impl cargo_aoc_take2::MaybeRunner
        for crate::__aoc::Aoc<crate::__aoc::Day1, crate::__aoc::Part1, crate::__aoc::Base>
    {
        type R = Self;

        fn into_runner(self) -> Option<Self::R> {
            Some(self)
        }
    }

    /// GENERATED BY CARGO-AOC
    /// Custom runner implementation
    impl cargo_aoc_take2::Runner
        for crate::__aoc::Aoc<crate::__aoc::Day1, crate::__aoc::Part1, crate::__aoc::Base>
    {
        fn run(
            &self,
            input: &str,
        ) -> cargo_aoc_take2::RunResult {
            use cargo_aoc_take2::from_input::FromInput;
            use cargo_aoc_take2::RunResult;
            use std::time::Instant;

            let start = Instant::now();

            // Based on parameter type `&LinesVec<Parse<i32>>`, we can call `LinesVec::<Parse<i32>>::from_input`.
            let parsed_input = match LinesVec::<Parse<i32>>::from_input(input) {
                Ok(parsed_input) => parsed_input,
                Err(error) => return RunResult::GenerationFailed { error: error.into() },
            };
            let generation_completed = Instant::now();
            let generation = generation_completed - start;

            // Based on parameter type `&LinesVec<Parse<i32>>`, we know that we can pass a reference to parsed_input
            // and based on the return type (not a `Result`), we know that we don't need to match on the result.
            let solution = part1(&parsed_input);
            let execution = generation_completed.elapsed();
            RunResult::Success {
                solution: Box::new(solution),
                generation,
                execution,
            }
        }
    }

    // Based on : #[aoc(day1, part1, alt1)]
    fn part1_iter(
        LinesIter(mut iter): LinesIter<Parse<i32>>,
    ) -> Result<i32, ParseIntError> {
        iter.try_fold(0, |acc, n| Ok(acc + *n?))
    }

    /// GENERATED BY CARGO-AOC
    /// Replace the default `Runner` implementation by the custom one, see below
    impl cargo_aoc_take2::MaybeRunner
        for crate::__aoc::Aoc<crate::__aoc::Day1, crate::__aoc::Part1, crate::__aoc::Alt1>
    {
        type R = Self;

        fn into_runner(self) -> Option<Self::R> {
            Some(self)
        }
    }

    /// GENERATED BY CARGO-AOC
    /// Custom runner implementation
    impl cargo_aoc_take2::Runner
        for crate::__aoc::Aoc<crate::__aoc::Day1, crate::__aoc::Part1, crate::__aoc::Alt1>
    {
        fn run(
            &self,
            input: &str,
        ) -> cargo_aoc_take2::RunResult {
            use cargo_aoc_take2::from_input::FromInput;
            use cargo_aoc_take2::RunResult;
            use std::time::Instant;

            let start = Instant::now();

            // Based on parameter type `LinesIter<Parse<i32>>`, we can call `LinesIter::<Parse<i32>>::from_input`.
            let parsed_input = match LinesIter::<Parse<i32>>::from_input(input) {
                Ok(parsed_input) => parsed_input,
                Err(error) => return RunResult::GenerationFailed { error: error.into() },
            };
            let generation_completed = Instant::now();
            let generation = generation_completed - start;

            // Based on parameter type `LinesIter<Parse<i32>>`, we know that we can pass parsed_input ownership
            // and based on the return type (`Result`), we know that we do need to match on the result.
            let solution = match part1_iter(parsed_input) {
                Ok(solution) => solution,
                Err(error) => return RunResult::RunFailed { generation, error: error.into() },
            };
            let execution = generation_completed.elapsed();
            RunResult::Success {
                solution: Box::new(solution),
                generation,
                execution,
            }
        }
    }

    // #[aoc(day1, part1, alt2="Generation error")]
    fn part1_generation_error(LinesVec(freqs): &LinesVec<Parse<u32>>) -> u32 {
        freqs.iter().map(|Parse(n)| n).sum()
    }

    /// GENERATED BY CARGO-AOC
    /// Replace the default `Runner` implementation by the custom one, see below
    impl cargo_aoc_take2::MaybeRunner
    for crate::__aoc::Aoc<crate::__aoc::Day1, crate::__aoc::Part1, crate::__aoc::Alt2>
    {
        type R = Self;

        fn into_runner(mut self) -> Option<Self::R> {
            // This one sets a custom name for the alternative
            self.version_mut().set_name("Generation error");
            Some(self)
        }
    }

    /// GENERATED BY CARGO-AOC
    /// Custom runner implementation
    impl cargo_aoc_take2::Runner
    for crate::__aoc::Aoc<crate::__aoc::Day1, crate::__aoc::Part1, crate::__aoc::Alt2>
    {
        fn run(
            &self,
            input: &str,
        ) -> cargo_aoc_take2::RunResult {
            use cargo_aoc_take2::from_input::FromInput;
            use cargo_aoc_take2::RunResult;
            use std::time::Instant;

            let start = Instant::now();
            let parsed_input = match LinesVec::<Parse<u32>>::from_input(input) {
                Ok(parsed_input) => parsed_input,
                Err(error) => return RunResult::GenerationFailed { error: error.into() },
            };
            let generation_completed = Instant::now();
            let generation = generation_completed - start;
            let solution = part1_generation_error(&parsed_input);
            let execution = generation_completed.elapsed();
            RunResult::Success {
                solution: Box::new(solution),
                generation,
                execution,
            }
        }
    }

    // Based on #[aoc(day1, part2)]
    fn part2(LinesVec(freqs): &LinesVec<Parse<i32>>) -> i32 {
        let mut reached = HashSet::new();
        let mut sum = 0;

        reached.insert(sum);

        freqs
            .iter()
            .cycle()
            .take_while(|&&Parse(f)| {
                sum += f;
                reached.insert(sum)
            })
            .count();

        sum
    }

    /// GENERATED BY CARGO-AOC
    /// Replace the default `Runner` implementation by the custom one, see below
    impl cargo_aoc_take2::MaybeRunner
        for crate::__aoc::Aoc<crate::__aoc::Day1, crate::__aoc::Part2, crate::__aoc::Base>
    {
        type R = Self;

        fn into_runner(self) -> Option<Self::R> {
            Some(self)
        }
    }

    /// GENERATED BY CARGO-AOC
    /// Custom runner implementation
    impl cargo_aoc_take2::Runner
        for crate::__aoc::Aoc<crate::__aoc::Day1, crate::__aoc::Part2, crate::__aoc::Base>
    {
        fn run(
            &self,
            input: &str,
        ) -> cargo_aoc_take2::RunResult {
            use cargo_aoc_take2::from_input::FromInput;
            use cargo_aoc_take2::RunResult;
            use std::time::Instant;

            let start = Instant::now();
            let parsed_input = match LinesVec::<Parse<i32>>::from_input(input) {
                Ok(parsed_input) => parsed_input,
                Err(error) => return RunResult::GenerationFailed { error: error.into() },
            };
            let generation_completed = Instant::now();
            let generation = generation_completed - start;
            let solution = part2(&parsed_input);
            let execution = generation_completed.elapsed();
            RunResult::Success {
                solution: Box::new(solution),
                generation,
                execution,
            }
        }
    }

    // #[aoc(day1, part2, alt1=Fnv)]
    fn part2_fnv(LinesVec(freqs): &LinesVec<Parse<i32>>) -> i32 {
        let mut reached = FnvHashSet::default();
        let mut sum = 0;

        reached.insert(sum);

        freqs
            .iter()
            .cycle()
            .take_while(|&&Parse(f)| {
                sum += f;
                reached.insert(sum)
            })
            .count();

        sum
    }

    /// GENERATED BY CARGO-AOC
    /// Replace the default `Runner` implementation by the custom one, see below
    impl cargo_aoc_take2::MaybeRunner
        for crate::__aoc::Aoc<crate::__aoc::Day1, crate::__aoc::Part2, crate::__aoc::Alt1>
    {
        type R = Self;

        fn into_runner(mut self) -> Option<Self::R> {
            // This one sets a custom name for the alternative
            self.version_mut().set_name("Fnv");
            Some(self)
        }
    }

    /// GENERATED BY CARGO-AOC
    /// Custom runner implementation
    impl cargo_aoc_take2::Runner
        for crate::__aoc::Aoc<crate::__aoc::Day1, crate::__aoc::Part2, crate::__aoc::Alt1>
    {
        fn run(
            &self,
            input: &str,
        ) -> cargo_aoc_take2::RunResult {
            use cargo_aoc_take2::from_input::FromInput;
            use cargo_aoc_take2::RunResult;
            use std::time::Instant;

            let start = Instant::now();
            let parsed_input = match LinesVec::<Parse<i32>>::from_input(input) {
                Ok(parsed_input) => parsed_input,
                Err(error) => return RunResult::GenerationFailed { error: error.into() },
            };
            let generation_completed = Instant::now();
            let generation = generation_completed - start;
            let solution = part2_fnv(&parsed_input);
            let execution = generation_completed.elapsed();
            RunResult::Success {
                solution: Box::new(solution),
                generation,
                execution,
            }
        }
    }

    // #[aoc(day1, part2, alt2="Execution error")]
    fn part2_execution_error(LinesIter(iter): LinesIter<Parse<u32>>) -> Result<u32, ParseIntError> {
        let mut reached = FnvHashSet::default();
        let mut sum = 0;

        reached.insert(sum);

        for f in iter.cycle() {
            let Parse(f) = f?;
            sum += f;
            if !reached.insert(sum) {
                break;
            }
        }

        Ok(sum)
    }

    /// GENERATED BY CARGO-AOC
    /// Replace the default `Runner` implementation by the custom one, see below
    impl cargo_aoc_take2::MaybeRunner
    for crate::__aoc::Aoc<crate::__aoc::Day1, crate::__aoc::Part2, crate::__aoc::Alt2>
    {
        type R = Self;

        fn into_runner(mut self) -> Option<Self::R> {
            // This one sets a custom name for the alternative
            self.version_mut().set_name("Execution error");
            Some(self)
        }
    }

    /// GENERATED BY CARGO-AOC
    /// Custom runner implementation
    impl cargo_aoc_take2::Runner
    for crate::__aoc::Aoc<crate::__aoc::Day1, crate::__aoc::Part2, crate::__aoc::Alt2>
    {
        fn run(
            &self,
            input: &str,
        ) -> cargo_aoc_take2::RunResult {
            use cargo_aoc_take2::from_input::FromInput;
            use cargo_aoc_take2::RunResult;
            use std::time::Instant;

            let start = Instant::now();
            let parsed_input = match LinesIter::<Parse<u32>>::from_input(input) {
                Ok(parsed_input) => parsed_input,
                Err(error) => return RunResult::GenerationFailed { error: error.into() },
            };
            let generation_completed = Instant::now();
            let generation = generation_completed - start;
            let solution = match part2_execution_error(parsed_input) {
                Ok(solution) => solution,
                Err(error) => return RunResult::RunFailed { generation, error: error.into() },
            };
            let execution = generation_completed.elapsed();
            RunResult::Success {
                solution: Box::new(solution),
                generation,
                execution,
            }
        }
    }
}
