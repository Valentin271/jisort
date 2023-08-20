#![warn(clippy::todo, missing_docs)]
//! This crate provides utilities to sort Javascript & Typescript imports.
//!
//! Its main form of usage is a binary.
//!
//! You can also use it as a library to build on top of it.
//!
//! # Features
//!
//! | Feature | Description |
//! |---|---|
//! | `argh` | *Enabled by default.* Enable command line argument parsing with [argh](https://github.com/google/argh). |

pub mod app;
mod config;
pub mod error;
pub mod file;
mod parser;
pub mod prelude;
mod statements;

pub use config::Config;
use error::Error;

use crate::app::report::{Report, ReportTrait};

/// Run the import sorter.
///
/// This is a convenience method to create an [App](app::App) and print its [Report](app::report::Report).
///
/// This also checks for the [version](Config::version) & [list](Config::list) flag, unlike
/// [App::run](app::App::run) which does not.
///
/// # Example
///
/// ```
/// use jisort::prelude::*;
/// let config = Config::default();
/// let res = run(config);
/// ```
pub fn run(config: Config) -> Result<(), Error> {
    if config.version {
        println!("{}", app::VERSION);
        return Ok(());
    }

    let mut app = app::App::new(config.clone());

    if config.list {
        for entry in app.files()? {
            println!("{}", entry.path().display());
        }
        return Ok(());
    }

    let report = app.run()?;

    println!("{}", report);

    match report {
        Report::Check(report) if report.err() => Err("".into()),
        _ => Ok(()),
    }
}
