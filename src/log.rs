use std::fmt::{self, Display};
use std::error::Error;
use std::io;
/// Error wrapping a static string message explaining why parsing failed.
#[derive(Debug,Clone,Copy)]
pub struct InvalidData(&'static str);
impl InvalidData {
    /// Error wrapping a static string message explaining why parsing failed.
    pub fn new(c: &'static str) -> Self {
        Self(c)
    }
}
impl Display for InvalidData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl Error for InvalidData {
    fn description(&self) -> &str {
        self.0
    }
}
impl From<InvalidData> for io::Error {
    fn from(e: InvalidData) -> io::Error {
        io::Error::new(io::ErrorKind::InvalidData, e.0)
    }
}
