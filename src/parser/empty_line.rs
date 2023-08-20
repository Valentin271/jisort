use nom::{
    character::complete::{line_ending, space0},
    IResult,
};

/// Parse an empty line.
///
/// An empty line might consist of nothing or multiple spaces.
/// An empty line always has a line ending (`\n` or `\r\n`).
pub fn empty_line(input: &str) -> IResult<&str, ()> {
    let (input, _) = space0(input)?;
    let (input, _) = line_ending(input)?;

    Ok((input, ()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dont_parse_nothing() {
        let res = empty_line("");

        assert!(res.is_err());
    }

    #[test]
    fn parse_newline() {
        let (rest, _) = empty_line("\n").unwrap();

        assert_eq!(rest, "");
    }

    #[test]
    fn parse_carriage_return() {
        let (rest, _) = empty_line("\r\n").unwrap();

        assert_eq!(rest, "");
    }

    #[test]
    fn parse_with_spaces() {
        let (rest, _) = empty_line("  \n").unwrap();

        assert_eq!(rest, "");
    }

    #[test]
    fn parse_with_tab() {
        let (rest, _) = empty_line("\t\n").unwrap();

        assert_eq!(rest, "");
    }
}
