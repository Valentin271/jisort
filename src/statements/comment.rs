use super::Statement;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Comment {
    pub data: String,
    pub ty: CommentType,
}

impl Statement for Comment {
    fn code(&self) -> String {
        match self.ty {
            CommentType::Single => format!("//{}", self.data.trim_end()),
            CommentType::Multi => format!("/*{}*/", self.data),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CommentType {
    Single,
    Multi,
}
