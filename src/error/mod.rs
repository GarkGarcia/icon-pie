use crate::{Output};
use icon_baker::AsSize;
use std::{io, fmt::{self, Formatter, Display}};
use crossterm::{style, Color};

mod show;

#[derive(Debug)]
pub enum Error {
    Syntax(SyntaxError),
    AlreadyIncluded(u32),
    InvalidDimensions(u32),
    Io(io::Error, Output)
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SyntaxError {
    UnexpectedToken(usize),
    UnexpectedEnd
}

impl Into<io::Error> for Error {
    fn into(self) -> io::Error {
        match self {
            Error::InvalidDimensions(_) | Error::AlreadyIncluded(_) | Error::Syntax(_)  => io::Error::from(io::ErrorKind::InvalidInput),
            Error::Io(err, _) => io::Error::from(err.kind()),
        }
    }
}

impl<K: AsSize> From<icon_baker::Error<K>> for Error {
    fn from(err: icon_baker::Error<K>) -> Self {
        match err {
            icon_baker::Error::AlreadyIncluded(key) => Error::AlreadyIncluded(key.as_size()),
            icon_baker::Error::Io(_) => unimplemented!(),
            icon_baker::Error::MismatchedDimensions(_, _) => unreachable!(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self {
            Error::AlreadyIncluded(s) => write!(
                f,
                "{0} The icon already contains a {1}x{1} entry.",
                style("[Already Included]").with(Color::Red),
                s
            ),
            Error::InvalidDimensions(s) => write!(
                f,
                "{0} Icons of {1}x{1} dimensions are not supported.",
                style("[Invalid Dimensions]").with(Color::Red),
                s
            ),
            Error::Io(err, path) => show::io(f, err, path.clone()),
            Error::Syntax(err) => err.fmt(f),
        }
    }
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