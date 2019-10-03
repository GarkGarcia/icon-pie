use std::{path::PathBuf, iter::{Iterator, Peekable, Enumerate}, slice::Iter};
use crate::{command::Command, ResamplingFilter, Entries, Output, syntax, error::{Error, SyntaxError}};
use token::{Flag};
pub use token::Token;

mod token;
type TokenStream<'a> = Peekable<Enumerate<Iter<'a, Token>>>;

pub fn args() -> Result<Command, Error> {
    let args = crate::args();

    if args.is_empty() { return Ok(Command::Help); }

    let n_entries = args.iter().fold(0, |sum, arg| if arg == "-e" { sum + 1 } else { sum });
    let mut entries = Vec::with_capacity(n_entries);

    let tokens = tokens(args);
    let mut it = tokens.iter().enumerate().peekable();

    while let Some(&(c, token)) = it.peek() {
        match token {
            Token::Flag(Flag::Entry)   => add_entry(&mut it, &mut entries)?,
            Token::Flag(Flag::Help)    => return expect_end(&mut it, Command::Help),
            Token::Flag(Flag::Version) => return expect_end(&mut it, Command::Version),
            Token::Flag(Flag::Ico) | Token::Flag(Flag::Icns) | Token::Flag(Flag::Png)
                => return command(&mut it, entries),
            _   => return syntax!(SyntaxError::UnexpectedToken(c))
        }
    }

    syntax!(SyntaxError::UnexpectedEnd)
}

#[inline]
fn tokens<'a>(args: Vec<String>) -> Vec<Token> {
    args.iter().map(|arg| Token::from(arg.as_ref())).collect()
}

fn entry(it: &mut TokenStream, entries: &mut Entries, path: &PathBuf) -> Result<(), Error> {
    // TODO Preallocate this Vec
    let mut sizes = Vec::with_capacity(0);

    it.next();
    match it.peek() {
        Some(&(_, Token::Size(_))) => while let Some(&(_, Token::Size(size))) = it.peek() {
            it.next();
            sizes.push(*size);
        },
        Some(&(c, _)) => return syntax!(SyntaxError::UnexpectedToken(c)),
        None          => return syntax!(SyntaxError::UnexpectedEnd)
    }

    let filter = filter(it)?;

    for size in sizes {
        entries.push((size, path.clone(), filter));
    }

    Ok(())
}

fn add_entry(it: &mut TokenStream, entries: &mut Entries) -> Result<(), Error> {
    it.next();
    match it.peek() {
        Some(&(_, Token::Path(path))) => entry(it, entries, path),
        Some(&(c, _)) => syntax!(SyntaxError::UnexpectedToken(c)),
        None          => syntax!(SyntaxError::UnexpectedEnd)
    }
}

fn filter(it: &mut TokenStream) -> Result<ResamplingFilter, Error> {
    if let Some((_, Token::Flag(Flag::Resample))) = it.peek() {
        it.next();
        match it.peek() {
            Some(&(_, Token::Filter(filter))) => { it.next(); return Ok(*filter); },
            Some(&(c, _)) => return syntax!(SyntaxError::UnexpectedToken(c)),
            None          => return syntax!(SyntaxError::UnexpectedEnd)
        }
    }

    Ok(ResamplingFilter::Nearest)
}

fn command(it: &mut TokenStream, entries: Entries) -> Result<Command, Error> {
    let icon_type = icon_type(it)?;

    it.next();
    match it.peek() {
        Some(&(_, Token::Path(path))) => expect_end(it, icon!(entries, icon_type, path.clone())),
        Some(&(c, _)) => syntax!(SyntaxError::UnexpectedToken(c)),
        None          => Ok(icon!(entries, icon_type))
    }
}

fn icon_type(it: &mut TokenStream) -> Result<IconType, Error> {
    match it.peek() {
        Some(&(_, Token::Flag(Flag::Ico)))  => Ok(IconType::Ico),
        Some(&(_, Token::Flag(Flag::Icns))) => Ok(IconType::Icns),
        Some(&(_, Token::Flag(Flag::Png)))  => Ok(IconType::PngSequence),
        Some(&(c, _))                       => syntax!(SyntaxError::UnexpectedToken(c)),
        None                                => syntax!(SyntaxError::UnexpectedEnd)
    }
}

fn expect_end(it: &mut TokenStream, command: Command) -> Result<Command, Error> {
    it.next();
    match it.peek() {
        Some(&(c, _)) => syntax!(SyntaxError::UnexpectedToken(c)),
        None => Ok(command)
    }
}