use std::fmt::Display;

use crate::file::File;

use super::ReportTrait;

/// This represents the result of a *fix* run.
///
/// Files can be:
///
/// - left unchanged when they are correctly formated.
/// - sorted when when they are not dangerous or when the `force` flag is enabled
/// - marked as dangerous when they are and the `force` flag is not enabled
#[derive(Debug, Default)]
pub struct FixReport {
    /// Unchanged files.
    ///
    /// Files that have not been fixed because they don't need to be.
    pub unchanged_files: Vec<File>,
    /// Files that have been sorted because their imports were unordered.
    pub sorted_files: Vec<File>,
    /// Files marked dangerous likely because they have comments between their imports.
    pub dangerous_files: Vec<File>,
}

impl ReportTrait for FixReport {
    fn len(&self) -> usize {
        self.sorted_files.len() + self.unchanged_files.len() + self.dangerous_files.len()
    }

    fn all(&self) -> Vec<File> {
        let mut res = Vec::with_capacity(self.len());

        res.append(&mut self.sorted_files.clone());
        res.append(&mut self.unchanged_files.clone());
        res.append(&mut self.dangerous_files.clone());

        res
    }

    fn err(&self) -> bool {
        !self.dangerous_files.is_empty()
    }
}

impl Display for FixReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} files checked. {} files sorted. {err}{} dangerous files.\x1b[m",
            self.len(),
            self.sorted_files.len(),
            self.dangerous_files.len(),
            err = if self.err() { "\x1b[31m" } else { "" }
        )
    }
}
