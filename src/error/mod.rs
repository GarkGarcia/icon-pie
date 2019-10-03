use crate::Output;
use std::{io, path::PathBuf, fmt::{self, Formatter, Display}};
use icon_baker::AsSize;
use crossterm::{style, Color};

mod syntax;
mod file;

pub use syntax::SyntaxError;
pub use file::FileError;

#[derive(Debug)]
pub enum Error {
    Syntax(SyntaxError),
    AlreadyIncluded(u32),
    InvalidDimensions(u32),
    File(FileError),
    Output(io::Error, Output)
}

impl Error {
    pub fn from_baker<K: AsSize>(err: icon_baker::Error<K>, path: PathBuf) -> Self {
        match err {
            icon_baker::Error::AlreadyIncluded(key) => Error::AlreadyIncluded(key.as_size()),
            icon_baker::Error::Io(err) => Error::File(FileError(err, path)),
            icon_baker::Error::MismatchedDimensions(_, _) => unreachable!()
        }
    }
}

impl Into<io::Error> for Error {
    fn into(self) -> io::Error {
        match self {
            Error::File(FileError(err, _)) | Error::Output(err, _) => err,
            _  => io::Error::from(io::ErrorKind::InvalidInput),
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
            Error::Output(err, output) => unimplemented!(),
            Error::File(err) => err.fmt(f),
            Error::Syntax(err) => err.fmt(f),
        }
    }
}