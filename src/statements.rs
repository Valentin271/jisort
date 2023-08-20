mod comment;
mod imports;

use std::fmt::Display;

pub use comment::*;
pub use imports::*;

pub trait Statement {
    fn code(&self) -> String;
}

impl Display for dyn Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code())
    }
}
