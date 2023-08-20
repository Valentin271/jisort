use std::collections::HashMap;

use crate::statements::{ImportStatement, Statement};

/// This is the data resulting from a parsed [File](super::File).
pub struct FileData {
    pub(super) imports: Vec<ImportStatement>,
    pub(super) statements: HashMap<usize, Vec<Box<dyn Statement>>>,
    pub(super) rest: String,
}

impl FileData {
    /// Creates a new [FileData] with default values.
    pub(super) fn new() -> Self {
        Self {
            imports: Default::default(),
            statements: Default::default(),
            rest: Default::default(),
        }
    }
    /// Tells whether this file is dangerous to format.
    ///
    /// Formating is dangerous when there are other statements in-between imports,
    /// like comments.
    pub fn is_dangerous(&self) -> bool {
        !self.statements.is_empty()
    }
}
