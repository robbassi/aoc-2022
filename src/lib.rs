pub mod result {
    use std::fmt::Debug;

    #[derive(Debug)]
    pub enum AocError {
        ParseError { got: String, expected: String },
        ErrResult { err: String },
        EmptyOption,
    }

    impl AocError {
        pub fn parse_error<T>(got: String, expected: String) -> AocResult<T> {
            Err(AocError::ParseError { got, expected })
        }
    }

    pub type AocResult<T> = Result<T, AocError>;

    pub trait AocResultT<T> {
        fn lift(&self) -> AocResult<T>;
    }

    impl<T, E> AocResultT<T> for Result<T, E> where T: Clone, E: Debug {
        fn lift(&self) -> AocResult<T> {
            match self {
                Ok(t) => Ok(t.clone()),
                Err(e) => {
                    let msg = format!("{e:?}");
                    Err(AocError::ErrResult { err: msg })
                },
            }
        }
    }

    impl<T> AocResultT<T> for Option<T> where T: Clone {
        fn lift(&self) -> AocResult<T> {
            match self {
                Some(a) => Ok(a.clone()),
                None => Err(AocError::EmptyOption),
            }
        }
    }
}

pub mod io {
    use std::io;
    use std::io::BufRead;

    pub fn read_input() -> Vec<String> {
        io::stdin()
            .lock()
            .lines()
            .map(Result::unwrap)
            .collect()
    }
}
