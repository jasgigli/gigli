//! Standard library: Result<T, E> for Gigli

#[derive(Debug, Clone, PartialEq)]
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    /// Returns true if the result is Ok.
    pub fn is_ok(&self) -> bool {
        matches!(self, Result::Ok(_))
    }

    /// Returns true if the result is Err.
    pub fn is_err(&self) -> bool {
        matches!(self, Result::Err(_))
    }

    /// Unwraps the value, panicking if Err.
    pub fn unwrap(self) -> T {
        match self {
            Result::Ok(v) => v,
            Result::Err(_) => panic!("called `Result::unwrap()` on an `Err` value"),
        }
    }

    /// Unwraps the value or returns the provided default.
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Result::Ok(v) => v,
            Result::Err(_) => default,
        }
    }

    /// Maps a Result<T, E> to Result<U, E> by applying a function to the contained value.
    pub fn map<U, F>(self, f: F) -> Result<U, E>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Result::Ok(v) => Result::Ok(f(v)),
            Result::Err(e) => Result::Err(e),
        }
    }

    /// Maps a Result<T, E> to Result<T, F> by applying a function to the error.
    pub fn map_err<FN, F>(self, f: FN) -> Result<T, F>
    where
        FN: FnOnce(E) -> F,
    {
        match self {
            Result::Ok(v) => Result::Ok(v),
            Result::Err(e) => Result::Err(f(e)),
        }
    }

    /// Returns Err if the result is Err, otherwise calls f with the contained value and returns the result.
    pub fn and_then<U, FN>(self, f: FN) -> Result<U, E>
    where
        FN: FnOnce(T) -> Result<U, E>,
    {
        match self {
            Result::Ok(v) => f(v),
            Result::Err(e) => Result::Err(e),
        }
    }
}
