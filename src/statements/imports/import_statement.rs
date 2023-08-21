use std::{cmp::Ordering, fmt::Debug};

use crate::statements::{Comment, Statement};

use super::ImportType;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ImportStatement {
    pub identifiers: Option<String>,
    pub module: String,
    pub comment: Option<Comment>,
}

impl ImportStatement {
    pub fn ty(&self) -> ImportType {
        ImportType::from(self.module.as_str())
    }
}

impl PartialOrd for ImportStatement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.ty(), &self.module).partial_cmp(&(other.ty(), &other.module))
    }
}

impl Ord for ImportStatement {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.ty(), &self.module).cmp(&(other.ty(), &other.module))
    }
}

impl Statement for ImportStatement {
    fn code(&self) -> String {
        if let Some(id) = &self.identifiers {
            format!(
                "import {} from '{}';{}",
                id,
                self.module,
                self.comment
                    .as_ref()
                    .map_or(String::new(), |c| " ".to_owned() + &c.code())
            )
        } else {
            format!(
                "import '{}';{}",
                self.module,
                self.comment
                    .as_ref()
                    .map_or(String::new(), |c| " ".to_owned() + &c.code())
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_sort() {
        let import1 = ImportStatement {
            identifiers: None,
            module: "abc".to_owned(),
            comment: None,
        };
        let import2 = ImportStatement {
            identifiers: None,
            module: "xyz".to_owned(),
            comment: None,
        };

        assert!(import1 < import2);
    }
}
