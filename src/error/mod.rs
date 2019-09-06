use crate::{Output};
use std::{io, fmt::{self, Formatter, Display}};

mod show;

#[derive(Debug)]
pub enum Error {
    Syntax(SyntaxError),
    IconBaker(icon_baker::Error),
    Io(io::Error, Output)
}

#[derive(Clone, Debug, PartialEq)]
pub enum SyntaxError {
    UnexpectedToken(usize),
    UnexpectedEnd
}

impl Into<io::Error> for Error {
    fn into(self) -> io::Error {
        match self {
            Error::Syntax(_)  => io::Error::from(io::ErrorKind::InvalidInput),
            Error::Io(err, _) => io::Error::from(err.kind()),
            Error::IconBaker(err) => err.into()
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self {
            Error::Syntax(err)    => show::syntax(f, err),
            Error::IconBaker(err) => show::icon_baker(f, err),
            Error::Io(err, path)  => show::io(f, err, path.clone())
        }
    }
}