use std::fmt::Display;

use crate::file::File;

use super::ReportTrait;

/// Represents the result of a *check* run.
///
/// Imports can either be sorted correctly or contain errors.
#[derive(Debug, Default)]
pub struct CheckReport {
    /// Files that were checked and contains no error.
    pub ok_files: Vec<File>,
    /// Files which imports are not sorted properly.
    pub errored_files: Vec<File>,
}

impl ReportTrait for CheckReport {
    fn len(&self) -> usize {
        self.ok_files.len() + self.errored_files.len()
    }

    fn all(&self) -> Vec<File> {
        let mut res = Vec::with_capacity(self.len());

        res.append(&mut self.ok_files.clone());
        res.append(&mut self.errored_files.clone());

        res
    }

    fn err(&self) -> bool {
        !self.errored_files.is_empty()
    }
}

impl Display for CheckReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} files checked. {} files ok. {error}{} files badly sorted.\x1b[m",
            self.len(),
            self.ok_files.len(),
            self.errored_files.len(),
            error = if self.err() { "\x1b[31m" } else { "" }
        )
    }
}
