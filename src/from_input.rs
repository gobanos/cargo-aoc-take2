pub use lines::{LinesIter, LinesVec};
pub use parse::Parse;
pub use str_ext::Trim;

pub trait FromInput<'input>: Sized {
    type Err;
    fn from_input(input: &'input str) -> Result<Self, Self::Err>;
}

mod parse {
    use crate::from_input::FromInput;
    use std::ops::{Deref, DerefMut};
    use std::str::FromStr;

    pub struct Parse<T>(pub T);

    impl<'input, T> FromInput<'input> for Parse<T>
    where
        T: FromStr,
    {
        type Err = T::Err;

        fn from_input(input: &'input str) -> Result<Self, Self::Err> {
            input.parse().map(Parse)
        }
    }

    impl<T> Deref for Parse<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T> DerefMut for Parse<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
}

mod str_ext {
    use crate::from_input::FromInput;
    use std::convert::Infallible;
    use std::ops::Deref;

    impl<'input> FromInput<'input> for &'input str {
        type Err = Infallible;

        fn from_input(input: &'input str) -> Result<Self, Self::Err> {
            Ok(input)
        }
    }

    pub struct Trim<'input>(pub &'input str);

    impl<'input> FromInput<'input> for Trim<'input> {
        type Err = Infallible;

        fn from_input(input: &'input str) -> Result<Self, Self::Err> {
            Ok(Trim(input.trim()))
        }
    }

    impl Deref for Trim<'_> {
        type Target = str;

        fn deref(&self) -> &Self::Target {
            self.0
        }
    }
}

mod lines {
    use crate::from_input::FromInput;
    use std::convert::Infallible;
    use std::iter::Map;
    use std::ops::{Deref, DerefMut};
    use std::str::Lines;

    pub struct LinesVec<T>(pub Vec<T>);
    impl<'input, T> FromInput<'input> for LinesVec<T>
    where
        T: FromInput<'input>,
    {
        type Err = T::Err;

        fn from_input(input: &'input str) -> Result<Self, Self::Err> {
            input
                .lines()
                .map(T::from_input)
                .collect::<Result<_, _>>()
                .map(LinesVec)
        }
    }

    impl<T> Deref for LinesVec<T> {
        type Target = Vec<T>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T> DerefMut for LinesVec<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    type FromInputSignature<'input, T> = fn(&'input str) -> Result<T, <T as FromInput>::Err>;

    pub struct LinesIter<'input, T: FromInput<'input>>(
        pub Map<Lines<'input>, FromInputSignature<'input, T>>,
    );
    impl<'input, T> FromInput<'input> for LinesIter<'input, T>
    where
        T: FromInput<'input> + 'input,
    {
        type Err = Infallible;

        fn from_input(input: &'input str) -> Result<Self, Self::Err> {
            Ok(LinesIter(input.lines().map(T::from_input)))
        }
    }
}
