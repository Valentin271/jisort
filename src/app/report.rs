//! This module contains report related structures.
//!
//! A report is a set of data which sums up a run.

mod check_report;
mod fix_report;

use std::fmt::Display;

pub use check_report::CheckReport;
pub use fix_report::FixReport;

use crate::file::File;

/// This is a wrapper around all reports this app can produce.
///
/// It implements [ReportTrait] to easily access common methods.
/// It also implements the [Display] trait so you can print a report, the implementation delegates
/// to the print to the wrapped reports.
///
/// # Example
/// ```
/// # use jisort::prelude::*;
/// # use jisort::app::report::ReportTrait;
/// let report = App::new(Config::default()).run().unwrap();
///
/// if report.err() {
///     println!("{} errors!", report.len());
/// }
///
/// println!("{}", report);
/// ```
pub enum Report {
    /// A report over a *fix* run.
    Fix(FixReport),
    /// A report over a *check* run.
    Check(CheckReport),
}

impl ReportTrait for Report {
    fn len(&self) -> usize {
        match self {
            Report::Fix(report) => report.len(),
            Report::Check(report) => report.len(),
        }
    }

    fn all(&self) -> Vec<File> {
        match self {
            Report::Fix(report) => report.all(),
            Report::Check(report) => report.all(),
        }
    }

    fn err(&self) -> bool {
        match self {
            Report::Fix(report) => report.err(),
            Report::Check(report) => report.err(),
        }
    }
}

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Report::Fix(report) => report.fmt(f),
            Report::Check(report) => report.fmt(f),
        }
    }
}

/// The main report type.
///
/// Every report implement this trait.
pub trait ReportTrait {
    /// Total number of processed files.
    fn len(&self) -> usize;
    /// Whether the run processed files or not.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Return every file processed.
    ///
    /// NOTE: This method is not optimal as it clones the report data into one [Vec].
    fn all(&self) -> Vec<File>;

    /// Tells whether the report is ok (no errors happened).
    ///
    /// See [ReportTrait::err] for mor details. This is the opposite.
    fn ok(&self) -> bool {
        !self.err()
    }
    /// Tells whether the report contains error.
    ///
    /// This is not a hard error. This is meant to represent normal check failure.o
    fn err(&self) -> bool;
}
