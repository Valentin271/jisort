//! Holds file related data.
//!
//! [File] is the main datatype of jisort.
//! It enabled [fixing](File::fix) & [checking](File::check) a file.

mod filedata;
use std::{fmt::Display, fs, path::PathBuf};

use crate::{
    parser::{comment, empty_line, import_statement},
    statements::{ImportType, Statement},
    Error,
};

pub use self::filedata::FileData;

/// Represents a file that can be parsed, fixed or checked.
#[derive(Debug, Clone)]
pub struct File {
    path: PathBuf,
}

impl File {
    /// Create a new [File] from its path.
    pub fn new<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self { path: path.into() }
    }

    /// Fix the file import order.
    ///
    /// This does **not** take into account [Config::force](crate::Config::force), in other words
    /// this will also format dangerous files.
    pub fn fix(&self, data: &FileData) -> Result<(), Error> {
        let mut imports = data.imports.clone();
        imports.sort();

        let mut text_imports = String::new();
        let mut last_import_type = imports
            .first()
            .map_or(ImportType::default(), |imp| imp.ty());

        for import in &imports {
            if last_import_type != import.ty() {
                last_import_type = import.ty();
                text_imports.push('\n');
            }
            text_imports.push_str(&import.code());
            text_imports.push('\n');
        }

        if !imports.is_empty() {
            text_imports.push('\n');
        }

        let content = text_imports + &data.rest.clone();

        fs::write(self.path.clone(), content)?;

        Ok(())
    }

    /// Check whether imports are correctly sorted in this file.
    ///
    /// Return [true] if they are, [false] otherwise.
    #[must_use]
    pub fn check(&self, data: &FileData) -> bool {
        let mut imports = data.imports.clone();
        imports.sort();

        data.imports == imports
    }

    /// Get the import data.
    pub fn parse(&self) -> Result<FileData, Error> {
        let mut program = fs::read_to_string(&self.path)?;
        let mut data = FileData::new();

        loop {
            if let Ok((input, import)) = import_statement(&program) {
                data.imports.push(import);
                program = input.to_owned();
            } else if let Ok((input, comment)) = comment(&program) {
                if let Some(tmp) = data.statements.get_mut(&data.imports.len()) {
                    tmp.push(Box::new(comment));
                } else {
                    data.statements
                        .insert(data.imports.len(), vec![Box::new(comment)]);
                }
                program = input.to_owned();
            } else if let Ok((input, _)) = empty_line(&program) {
                program = input.to_owned();
            } else {
                data.rest = program;
                break;
            }
        }

        Ok(data)
    }

    /// Gets the file path
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path().to_str().ok_or(std::fmt::Error)?)
    }
}

impl From<File> for PathBuf {
    fn from(value: File) -> Self {
        Self::from(value.path())
    }
}