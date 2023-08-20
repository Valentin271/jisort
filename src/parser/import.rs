use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_until1},
    character::complete::{char, none_of, one_of, space0, space1},
    combinator::{cond, opt, peek},
    sequence::delimited,
    IResult,
};

use crate::statements::ImportStatement;

use super::comment::comment;

/// Tries to match a JS string.
///
/// This function does not match a real JS string,
/// but rather a string that can take place in an import statement.
///
/// # Example
///
/// - 'foo'
/// - "bar"
fn js_string(input: &str) -> IResult<&str, &str> {
    let (input, string) = delimited(
        alt((char('\''), char('"'))),
        escaped(none_of(r#"\"'"#), '\\', one_of(r#"'"n\"#)),
        alt((char('\''), char('"'))),
    )(input)?;

    Ok((input, string))
}

/// Tries to parse a js import statement.
pub fn import_statement(input: &str) -> IResult<&str, ImportStatement> {
    // import keyword
    let (input, _) = tag("import")(input)?;
    let (input, _) = space1(input)?;

    let direct_import = peek(js_string)(input).is_ok();

    // identifier and from keyword
    let (input, identifier) = cond(!direct_import, take_until1("from"))(input)?;
    let (input, _) = cond(identifier.is_some(), tag("from"))(input)?;
    let (input, _) = space0(input)?;

    // module
    let (input, module) = js_string(input)?;

    // end
    let (input, _) = opt(char(';'))(input)?;
    let (input, comment) = opt(comment)(input)?;

    Ok((
        input,
        ImportStatement {
            identifiers: identifier.map(|s| s.trim().to_owned()),
            module: module.to_owned(),
            comment,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    mod js_string {
        use super::*;

        #[test]
        fn simple_quote() {
            let (rest, string) = js_string("'foo'").unwrap();

            assert_eq!(rest, "");
            assert_eq!(string, "foo");
        }

        #[test]
        fn double_quote() {
            let (rest, string) = js_string(r#""foo""#).unwrap();

            assert_eq!(rest, "");
            assert_eq!(string, "foo");
        }

        #[test]
        fn escaped() {
            let (rest, string) = js_string(r#"'f\'oo'"#).unwrap();

            assert_eq!(rest, "");
            assert_eq!(string, r#"f\'oo"#);
        }
    }

    mod import_statement {
        use super::*;

        #[test]
        fn basic_import_from() {
            let (rest, import) = import_statement("import x from 'a-module';").unwrap();

            assert_eq!(rest, "");

            assert_eq!(
                import,
                ImportStatement {
                    identifiers: Some("x".to_owned()),
                    module: "a-module".to_owned(),
                    comment: None
                }
            )
        }

        #[test]
        fn import_from_destructuring() {
            let (rest, import) = import_statement("import { a, b } from 'a-module';").unwrap();

            assert_eq!(rest, "");

            assert_eq!(
                import,
                ImportStatement {
                    identifiers: Some("{ a, b }".to_owned()),
                    module: "a-module".to_owned(),
                    comment: None
                }
            )
        }

        #[test]
        fn import_without_from() {
            let (rest, import) = import_statement("import 'style.css';").unwrap();

            assert_eq!(rest, "");

            assert_eq!(
                import,
                ImportStatement {
                    identifiers: None,
                    module: "style.css".to_owned(),
                    comment: None
                }
            )
        }

        /// Tests an import without from, but with a second from import after it.
        #[test]
        fn import_simple_from_next_line() {
            let (rest, import) =
                import_statement("import 'style.css';\nimport titi from 'toto';").unwrap();

            assert_eq!(rest, "\nimport titi from 'toto';");

            assert_eq!(
                import,
                ImportStatement {
                    identifiers: None,
                    module: "style.css".to_owned(),
                    comment: None
                }
            )
        }

        /// Tests an import with some text after the semicolon.
        ///
        /// Notice the end spaces have been trimmed, but not the leading ones.
        #[test]
        fn import_with_comment() {
            let (rest, import) =
                import_statement("import x from 'z'; // this is an import  ").unwrap();

            assert_eq!(rest, "");

            assert_eq!(
                import,
                ImportStatement {
                    identifiers: Some("x".to_owned()),
                    module: "z".to_owned(),
                    comment: Some(crate::statements::Comment {
                        data: " this is an import  ".to_owned(),
                        ty: crate::statements::CommentType::Single
                    })
                }
            )
        }

        #[test]
        fn import_without_semicolon() {
            let (rest, import) = import_statement("import x from 'z'").unwrap();

            assert_eq!(rest, "");

            assert_eq!(
                import,
                ImportStatement {
                    identifiers: Some("x".to_owned()),
                    module: "z".to_owned(),
                    comment: None
                }
            )
        }
    }
}
