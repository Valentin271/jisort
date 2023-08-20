use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until},
    character::complete::space0,
    sequence::delimited,
    IResult,
};

use crate::statements::{Comment, CommentType};

/// Matches a single line comment
pub fn single_line_comment(input: &str) -> IResult<&str, Comment> {
    let (input, _) = space0(input)?;
    let (input, _) = tag("//")(input)?;
    let (input, comment) = is_not("\n\r")(input)?;

    let comment = Comment {
        data: comment.to_owned(),
        ty: CommentType::Single,
    };
    Ok((input, comment))
}

/// Matches a multi line comment
pub fn multi_line_comment(input: &str) -> IResult<&str, Comment> {
    let (input, _) = space0(input)?;
    let (input, comment) = delimited(tag("/*"), take_until("*/"), tag("*/"))(input)?;

    let comment = Comment {
        data: comment.to_owned(),
        ty: CommentType::Multi,
    };
    Ok((input, comment))
}

/// Matches any comment, multi or single line
pub fn comment(input: &str) -> IResult<&str, Comment> {
    let (input, comment) = alt((single_line_comment, multi_line_comment))(input)?;

    Ok((input, comment))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_line() {
        let (rest, comment) = single_line_comment("// foo").unwrap();

        assert_eq!(rest, "");
        assert_eq!(comment.data, " foo")
    }

    /// Tests a multi line comment on one line
    #[test]
    fn multi_line_inline() {
        let (rest, comment) = multi_line_comment("/* foo */").unwrap();

        assert_eq!(rest, "");
        assert_eq!(comment.data, " foo ")
    }

    /// Tests a multi line comment on multiple lines
    #[test]
    fn multi_line() {
        let (rest, comment) = multi_line_comment("/*\n foo \n*/").unwrap();

        assert_eq!(rest, "");
        assert_eq!(comment.data, "\n foo \n")
    }
}
