/// Distinguish the types of imports.
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ImportType {
    /// Global import.
    ///
    /// This includes lodash, React
    #[default]
    Global,
    /// A scoped module.
    ///
    /// A scoped module is one that starts with `@`.
    ///
    /// # Caveats
    ///
    /// [Alias](ImportType::Alias) can also start with `@`.
    ScopedModule,
    /// Installed module
    ///
    /// Basically every module that does not fall into
    /// [Global](ImportType::Global) or [ScopedModule](ImportType::ScopedModule).
    Module,
    /// An alias for a import
    Alias,
    /// Project local import
    Local,
    /// Stylesheet import
    Style,
}

impl From<&str> for ImportType {
    fn from(value: &str) -> Self {
        if value == "@" || value.starts_with("@/") {
            Self::Alias
        } else if value.ends_with(".css") {
            Self::Style
        } else if value.starts_with("@") {
            Self::ScopedModule
        } else if value.starts_with('.') {
            Self::Local
        } else if vec!["react", "lodash", "prop-types"].contains(&value) {
            Self::Global
        } else {
            Self::Module
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ImportType::{self, *};

    #[test]
    fn global_before_style() {
        assert!(Global < Style);
    }

    #[test]
    fn module_before_local() {
        assert!(Module < Local);
    }

    mod from {
        use super::*;

        #[test]
        fn local_alias() {
            assert_eq!(ImportType::from("@/components"), Alias);
            assert_eq!(ImportType::from("@"), Alias);
            assert_eq!(ImportType::from("@/utils"), Alias);
        }

        #[test]
        fn scoped_module() {
            assert_eq!(ImportType::from("@react"), ScopedModule);
            assert_eq!(ImportType::from("@jvs-group/lib"), ScopedModule);
            assert_eq!(ImportType::from("@testing-library/react"), ScopedModule);
        }

        #[test]
        fn style() {
            // Style is more important than local import
            assert_eq!(ImportType::from("./style.css"), Style);
            // Style is more important than scoped module import
            assert_eq!(ImportType::from("@react/style.css"), Style);
            assert_eq!(ImportType::from("components/style.css"), Style);
        }

        #[test]
        fn local() {
            assert_eq!(ImportType::from("./third-party.js"), Local);
            assert_eq!(ImportType::from("../components/toolbar.js"), Local);
            assert_eq!(ImportType::from("../../utils/phone.js"), Local);
        }
    }
}
