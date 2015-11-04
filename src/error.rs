
use ::std;

/// An error returned when an unexpected nul is found in the string, slice or vector provided.
#[derive(Clone, PartialEq, Debug)]
pub struct NulError(usize, Option<Vec<u16>>);

/// An error returned when an expected nul is not found in the string, slice or vector provided.
#[derive(Clone, PartialEq, Debug)]
pub struct NoNulError(Option<Vec<u16>>);

pub fn nul(p: usize, s: Option<Vec<u16>>) -> NulError {
    NulError(p, s)
}

pub fn no_nul(s: Option<Vec<u16>>) -> NoNulError {
    NoNulError(s)
}

impl NulError {
    /// Return the position of the nul in u16 units.
    pub fn nul_position(&self) -> usize {
        self.0
    }

    /// Consume this error, returning the underlying Vec<u16> that contain the nul.
    /// This will provide the underlying Vec<u16> only when a Vec<u16> is passed in as a parameter
    /// and only when that Vec<u16> is consumed. Otherwise, this function returns None.
    pub fn into_vec(self) -> Option<Vec<u16>> {
        self.1
    }
}

impl std::fmt::Display for NulError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "nul found at position: {}", self.0)
    }
}

impl std::error::Error for NulError {
    fn description(&self) -> &str {
        "nul found"
    }
}

impl NoNulError {
    /// Consume this error, returning the underlying Vec<u16> that does not contain a nul.
    /// This will provide the underlying Vec<u16> only when a Vec<u16> is passed in as a parameter
    /// and only when that Vec<u16> is consumed. Otherwise, this function returns None.
    pub fn into_vec(self) -> Option<Vec<u16>> {
        self.0
    }
}

impl std::fmt::Display for NoNulError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "no nul found")
    }
}

impl std::error::Error for NoNulError {
    fn description(&self) -> &str {
        "no nul found"
    }
}

