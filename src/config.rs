use std::path::PathBuf;

#[cfg(feature = "argh")]
use argh::FromArgs;

/// A JavaScript / Typescript import sorter specifically built for JVS.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "argh", derive(FromArgs))]
pub struct Config {
    /// path where to search. Defaults to the current directory.
    #[cfg_attr(feature = "argh", argh(positional, default = "\".\".to_owned()"))]
    pub path: String,

    /// whether only check import order
    #[cfg_attr(feature = "argh", argh(switch))]
    pub check: bool,

    /// globs to match files against.
    /// Multiple globs can be separated by a comma `,`.
    #[cfg_attr(
        feature = "argh",
        argh(option, default = "globs_default()", from_str_fn(from_str_globs))
    )]
    pub globs: Vec<String>,

    /// list included files, then stops
    #[cfg_attr(feature = "argh", argh(switch))]
    pub list: bool,

    /// forces sorting when it is dangerous
    #[cfg_attr(feature = "argh", argh(switch))]
    pub force: bool,

    /// display version
    #[cfg_attr(feature = "argh", argh(switch, short = 'v'))]
    pub version: bool,
}

impl Config {
    /// Load config from cli arguments.
    ///
    /// Only available on the `argh` feature.
    #[cfg(feature = "argh")]
    pub fn from_cli() -> Self {
        argh::from_env()
    }

    /// Load config from environment variables
    pub fn from_env() -> Self {
        unimplemented!()
    }

    /// Load config from a config file
    pub fn from_file<P>(file: P) -> Self
    where
        P: Into<PathBuf>,
    {
        unimplemented!("args: {:?}", file.into())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            path: ".".to_owned(),
            check: false,
            globs: globs_default(),
            list: false,
            force: false,
            version: false,
        }
    }
}

/// Returns the default globs for js files.
pub fn globs_default() -> Vec<String> {
    vec![
        "**.js".to_owned(),
        "**.jsx".to_owned(),
        "**.ts".to_owned(),
        "**.tsx".to_owned(),
    ]
}

#[cfg(feature = "argh")]
fn from_str_globs(value: &str) -> Result<Vec<String>, String> {
    Ok(value.split(',').map(String::from).collect())
}
