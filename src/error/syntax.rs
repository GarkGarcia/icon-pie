use std::fmt::{self, Display, Formatter};
use crossterm::{style, Color};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SyntaxError {
    UnexpectedToken(usize),
    UnexpectedEnd
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let args = crate::args();

        match self {
            SyntaxError::UnexpectedToken(err_c) => write!(
                f,
                "{} {} {} {} {}",
                style("[Unexpected Token]").with(Color::Red),
                style("$ icon-pie").with(Color::Blue),
                style(args[..*err_c].join(" ")).with(Color::Blue),
                style(args[*err_c].clone()).with(Color::Red),
                style(args[(*err_c + 1)..].join(" ")).with(Color::Blue)
            ),
            SyntaxError::UnexpectedEnd => write!(
                f,
                "{} {} {} {}\nType {} for more details on IconBaker's usage.",
                style("[Expected Additional Tokens]").with(Color::Red),
                style("$ icon-pie").with(Color::Blue),
                style(args.join(" ")).with(Color::Blue),
                style("▂").with(Color::Red),
                style("icon-pie -h").with(Color::Blue)
            )
        }
    }
}