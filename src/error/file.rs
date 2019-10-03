use std::{io, path::PathBuf, fmt::{self, Display, Formatter}};
use crossterm::{style, Color};

#[derive(Debug)]
pub struct FileError(pub io::Error, pub PathBuf);

impl Display for FileError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.0.kind() {
            io::ErrorKind::NotFound => write!(
                f,
                "{} File {} could not be found on disk.",
                style("[IO Error]").with(Color::Red),
                style(self.1.display()).with(Color::Blue)
            ),
            io::ErrorKind::PermissionDenied => write!(
                f,
                "{} Permission denied. File {} is inaccessible.",
                style("[IO Error]").with(Color::Red),
                style(self.1.display()).with(Color::Blue)
            ),
            io::ErrorKind::AddrInUse | io::ErrorKind::AddrNotAvailable => write!(
                f,
                "{} File {} is unavaiable. Try closing any application that may be using it.",
                style("[IO Error]").with(Color::Red),
                style(self.1.display()).with(Color::Blue)
            ),
            io::ErrorKind::InvalidData | io::ErrorKind::InvalidInput => write!(
                f,
                "{} File {} couln't be parsed. This file may be corrupted.",
                style("[IO Error]").with(Color::Red),
                style(self.1.display()).with(Color::Blue)
            ),
            _ => write!(
                f,
                "{} {}.",
                style("[IO Error]").with(Color::Red),
                format(self.0.to_string())
            )
        }
    }
}

/// Makes sure errors message start with a capital letter and ends with '.'
fn format(txt: String) -> String {
    let mut output = String::with_capacity(txt.len());
    let mut chars = txt.chars();

    if let Some(ch) = chars.next() {
        if ch.is_lowercase() {
            output.extend(ch.to_uppercase());
        } else {
            output.push(ch);
        }
    }

    output.extend(chars);

    if !txt.ends_with('.') {
        output.push('.');
    }

    output
}