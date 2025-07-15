//! Standard library: Option<T> for Gigli

#[derive(Debug, Clone, PartialEq)]
pub enum Option<T> {
    Some(T),
    None,
}

impl<T> Option<T> {
    /// Returns true if the option is Some.
    pub fn is_some(&self) -> bool {
        matches!(self, Option::Some(_))
    }

    /// Returns true if the option is None.
    pub fn is_none(&self) -> bool {
        matches!(self, Option::None)
    }

    /// Unwraps the value, panicking if None.
    pub fn unwrap(self) -> T {
        match self {
            Option::Some(v) => v,
            Option::None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }

    /// Unwraps the value or returns the provided default.
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Option::Some(v) => v,
            Option::None => default,
        }
    }

    /// Maps an Option<T> to Option<U> by applying a function to the contained value.
    pub fn map<U, F>(self, f: F) -> Option<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Option::Some(v) => Option::Some(f(v)),
            Option::None => Option::None,
        }
    }

    /// Returns None if the option is None, otherwise calls f with the contained value and returns the result.
    pub fn and_then<U, F>(self, f: F) -> Option<U>
    where
        F: FnOnce(T) -> Option<U>,
    {
        match self {
            Option::Some(v) => f(v),
            Option::None => Option::None,
        }
    }
}
