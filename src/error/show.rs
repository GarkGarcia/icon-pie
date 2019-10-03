use std::{io, fmt::{self, Write}, path::PathBuf};
use crate::Output;
use super::SyntaxError;
use icon_baker::AsSize;
use crossterm::{style, Color};

pub fn syntax<W: Write>(w: &mut W, err: &SyntaxError) -> fmt::Result {
    let args = crate::args();

    match err {
        SyntaxError::UnexpectedToken(err_c) => write!(
            w,
            "{} {} {} {} {}",
            style("[Unexpected Token]").with(Color::Red),
            style("$ icon-pie").with(Color::Blue),
            style(args[..*err_c].join(" ")).with(Color::Blue),
            style(args[*err_c].clone()).with(Color::Red),
            style(args[(*err_c + 1)..].join(" ")).with(Color::Blue)
        ),
        SyntaxError::UnexpectedEnd => write!(
            w,
            "{} {} {} {}\nType {} for more details on IconBaker's usage.",
            style("[Expected Additional Tokens]").with(Color::Red),
            style("$ icon-pie").with(Color::Blue),
            style(args.join(" ")).with(Color::Blue),
            style("▂").with(Color::Red),
            style("icon-pie -h").with(Color::Blue)
        )
    }
}

pub fn icon_baker<W: Write, K: AsSize>(w: &mut W, err: &icon_baker::Error<K>) -> fmt::Result {
    match err {
        icon_baker::Error::AlreadyIncluded(key) => write!(
            w,
            "{0} The icon already contains a {1}x{1} entry.",
            style("[Already Included]").with(Color::Red),
            key.as_size()
        ),
        icon_baker::Error::Io(err) => write!(
            w,
            "{} {}",
            style("[IO Error]").with(Color::Red),
            format(err.to_string())
        ),
        icon_baker::Error::MismatchedDimensions(_, _) => unimplemented!()
    }
}

pub fn io<W: Write>(w: &mut W, err: &io::Error, out: Output) -> fmt::Result {
    match out {
        Output::Path(path) => file(w, err, path),
        Output::Stdout     => write!(
            w,
            "{} {}",
            style("[IO Error]").with(Color::Red),
            format(err.to_string())
        )
    }
}

fn file<W: Write>(w: &mut W, err: &io::Error, path: PathBuf) -> fmt::Result {
    match err.kind() {
        io::ErrorKind::NotFound => write!(
            w,
            "{} File {} could not be found on disk.",
            style("[IO Error]").with(Color::Red),
            style(path.display()).with(Color::Blue)
        ),
        io::ErrorKind::PermissionDenied => write!(
            w,
            "{} Permission denied. File {} is inaccessible.",
            style("[IO Error]").with(Color::Red),
            style(path.display()).with(Color::Blue)
        ),
        io::ErrorKind::AddrInUse | io::ErrorKind::AddrNotAvailable => write!(
            w,
            "{} File {} is unavaiable. Try closing any application that may be using it.",
            style("[IO Error]").with(Color::Red),
            style(path.display()).with(Color::Blue)
        ),
        io::ErrorKind::InvalidData | io::ErrorKind::InvalidInput => write!(
            w,
            "{} File {} couln't be parsed. This file may be corrupted.",
            style("[IO Error]").with(Color::Red),
            style(path.display()).with(Color::Blue)
        ),
        _ => write!(
            w,
            "{} {}.",
            style("[IO Error]").with(Color::Red),
            format(err.to_string())
        )
    }
}

// Makes sure errors message start with a capital letter and ends with '.'
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