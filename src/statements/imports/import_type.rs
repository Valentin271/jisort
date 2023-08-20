use super::ImportStatement;

/// Distinguish the types of imports.
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ImportType {
    /// Global import.
    ///
    /// This includes lodash, React
    #[default]
    Global,
    /// Installed module
    ///
    /// Basically every module
    Module,
    // An alias for a local import
    LocalAlias,
    /// Project local import
    Local,
    /// Stylesheet import
    Style,
}

impl From<&ImportStatement> for ImportType {
    fn from(value: &ImportStatement) -> Self {
        if value.module == "@" || value.module.starts_with("@/") {
            Self::LocalAlias
        } else if value.module.ends_with(".css") {
            Self::Style
        } else if value.module.starts_with('.') {
            Self::Local
        } else if vec!["react", "lodash", "prop-types"].contains(&value.module.as_str()) {
            Self::Global
        } else {
            Self::Module
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ImportType::*;

    #[test]
    fn global_before_style() {
        assert!(Global < Style);
    }

    #[test]
    fn module_before_local() {
        assert!(Module < Local);
    }
}
