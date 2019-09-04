use crate::{Output};
use std::io;

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

impl Error {
    pub fn exit_with(self) -> io::Error {
        match &self {
            Error::Syntax(err)    => show::syntax(err),
            Error::IconBaker(err) => show::icon_baker(err),
            Error::Io(err, path)  => show::io(err, path.clone())
        }

        self.into()
    }
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