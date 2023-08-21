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
        let imports_len = self.imports.len();
        self.statements.keys().any(|k| k > &0 && k < &imports_len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::statements::{Comment, CommentType};

    fn import(module: &str) -> ImportStatement {
        ImportStatement {
            identifiers: None,
            module: module.to_owned(),
            comment: None,
        }
    }

    fn comment(data: &str) -> Box<Comment> {
        Box::new(Comment {
            data: data.to_owned(),
            ty: CommentType::Single,
        })
    }

    #[test]
    fn empty_not_dangerous() {
        let data = FileData::new();
        assert!(!data.is_dangerous());
    }

    #[test]
    fn import_not_dangerous() {
        let mut data = FileData::new();
        data.imports.push(import("foo"));

        assert!(!data.is_dangerous());
    }

    #[test]
    fn statements_between_imports_dangerous() {
        let mut data = FileData::new();
        data.statements.insert(1, vec![comment("foo")]);
        data.imports.push(import("foo"));
        data.imports.push(import("foo"));

        assert!(data.is_dangerous());
    }

    #[test]
    fn statements_before_imports_not_danerous() {
        let mut data = FileData::new();
        data.statements.insert(0, vec![comment("foo")]);
        data.imports.push(import("foo"));

        assert!(!data.is_dangerous());
    }

    #[test]
    fn statements_after_imports_not_dangerous() {
        let mut data = FileData::new();
        data.statements.insert(1, vec![comment("foo")]);
        data.imports.push(import("foo"));

        assert!(!data.is_dangerous());
    }

    #[test]
    fn statements_around_imports_not_dangerous() {
        let mut data = FileData::new();
        data.statements.insert(0, vec![comment("foo")]);
        data.statements.insert(1, vec![comment("bar")]);
        data.imports.push(import("foo"));

        assert!(!data.is_dangerous());
    }

    #[test]
    fn only_statements_not_dangerous() {
        let mut data = FileData::new();
        data.statements.insert(0, vec![comment("foo")]);
        data.statements.insert(1, vec![comment("bar")]);

        assert!(!data.is_dangerous());
    }
}
