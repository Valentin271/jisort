//! This module is the main part of the crate.
//!
//! It provides the [App] struct.
//!
//! See also the [report] module.

pub mod report;

use std::path::PathBuf;

use ignore::{overrides::OverrideBuilder, WalkBuilder};

use crate::{error::Error, file::File, Config};
use report::Report;

use self::report::{CheckReport, FixReport, ReportTrait};

/// The crate version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
/// The crate **major** version.
///
/// See <https://semver.org/>
pub const VERSION_MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
/// The crate **minor** version.
///
/// See <https://semver.org/>
pub const VERSION_MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
/// The crate **patch** version.
///
/// See <https://semver.org/>
pub const VERSION_PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");

/// This is the main API entrypoint.
///
/// It represents the jisort application which can fix or check files.
#[derive(Default)]
pub struct App {
    config: Config,
}

impl App {
    /// Creates a new app from a [Config].
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Starts the app.
    ///
    /// Parse files specified by the globs in the [App]'s [Config]
    /// and run the appropriate scan.
    ///
    /// Does not take into account [version](Config::version) & [list](Config::list) flags.
    /// For this, see [run](crate::run).
    pub fn run(&mut self) -> Result<Report, Error> {
        let report = if self.config.check {
            Report::Check(self.check()?)
        } else {
            Report::Fix(self.fix()?)
        };

        Ok(report)
    }

    /// Check whether files imports are sorted correctly.
    ///
    /// # Command line
    ///
    /// This method is called by [App::run] if the `--check` flag is given.
    ///
    /// # Usage
    ///
    /// This is particularly useful in CI pipelines.
    pub fn check(&self) -> Result<CheckReport, Error> {
        let mut report = CheckReport::default();

        for file in self.files()? {
            let data = file.parse()?;

            if file.check(&data) {
                report.ok_files.push(file);
            } else {
                report.errored_files.push(file);
            }
        }

        Ok(report)
    }

    /// Fix badly sorted files in place.
    ///
    /// Dangerous files won't be sorted and this will be reported.
    /// Files are marked dangerous mainly if they contain comments between imports.
    ///
    /// # Command line
    ///
    /// Dangerous files can still be sorted with the `--force` flag.
    pub fn fix(&self) -> Result<FixReport, Error> {
        let mut report = FixReport::default();

        for file in self.files()? {
            let data = file.parse()?;

            if !data.is_dangerous() || self.config.force {
                file.fix(&data)?;
                report.sorted_files.push(file.clone());
            } else {
                report.dangerous_files.push(file.clone());
            }
        }

        if report.err() {
            Self::print_force_warning(&report.dangerous_files);
        }

        Ok(report)
    }

    /// Returns an iterator on the files that match the globs in the [App]'s [Config].
    pub fn files(&self) -> Result<impl Iterator<Item = File>, Error> {
        let mut globs = OverrideBuilder::new(self.config.path.clone());

        for glob in self.config.globs.clone() {
            globs.add(&glob)?;
        }

        let globs = globs.build()?;

        Ok(WalkBuilder::new(self.config.path.clone())
            .overrides(globs)
            .build()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .map(|entry| File::new(entry.path())))
    }

    fn print_force_warning<P>(files: &[P])
    where
        P: Into<PathBuf>,
    {
        eprintln!("\x1b[31mComments have been located between imports of {} files. Sorting is dangerous.\x1b[m", files.len());
        eprintln!("\x1b[36mUse \x1b[1m--force\x1b[0;36m to process those files anyway.\x1b[m");
        eprintln!();
    }
}
