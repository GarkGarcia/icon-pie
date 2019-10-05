use crate::{command::Command, ResamplingFilter, Entries, syntax, error::{Error, SyntaxError}};
use std::{path::PathBuf, iter::{Iterator, Peekable, Enumerate}, slice::Iter};
use icon_baker::Icon;

mod token;
mod ico;
mod icns;
mod favicon;

use token::{Flag, Token, Cmd};

type TokenStream<'a> = Peekable<Enumerate<Iter<'a, Token>>>;

pub fn args() -> Result<Command, Error> {
    let args = crate::args();

    if args.is_empty() { return Ok(Command::Help); }

    let tokens = tokens(args);
    let n_entries = tokens
        .iter()
        .fold(0, |sum, tk| if let Token::Flag(Flag::Entry) = tk { sum + 1 } else { sum });
    let mut it = tokens.iter().enumerate().peekable();

    match it.peek() {
        Some(&(_, Token::Command(Cmd::Ico))) => ico::parse(&mut it, n_entries),
        Some(&(_, Token::Command(Cmd::Icns))) => icns::parse(&mut it, n_entries),
        Some(&(_, Token::Command(Cmd::Favicon))) => favicon::parse(&mut it, n_entries),
        Some(&(_, Token::Flag(Flag::Help))) => expect_end(&mut it, Command::Help),
        Some(&(_, Token::Flag(Flag::Version))) => expect_end(&mut it, Command::Version),
        Some(&(c, _)) => syntax!(SyntaxError::UnexpectedToken(c)),
        None => syntax!(SyntaxError::UnexpectedEnd)
    }
}

#[inline]
fn tokens<'a>(args: Vec<String>) -> Vec<Token> {
    args.iter().map(|arg| Token::from(arg.as_ref())).collect()
}

fn add_entry<I: Icon, F: FnMut(&mut TokenStream, &mut Entries<<I as Icon>::Key>, &PathBuf) -> Result<(), Error>>(
    it: &mut TokenStream,
    entries: &mut Entries<<I as Icon>::Key>,
    mut adder: F,
) -> Result<(), Error> {
    it.next();
    match it.peek() {
        Some(&(_, Token::Path(path))) => adder(it, entries, path),
        Some(&(c, _)) => syntax!(SyntaxError::UnexpectedToken(c)),
        None => syntax!(SyntaxError::UnexpectedEnd)
    }
}

fn filter(it: &mut TokenStream) -> Result<ResamplingFilter, Error> {
    if let Some((_, Token::Flag(Flag::Resample))) = it.peek() {
        it.next();
        match it.peek() {
            Some(&(_, &Token::Filter(filter))) => { it.next(); return Ok(filter); },
            Some(&(c, _)) => return syntax!(SyntaxError::UnexpectedToken(c)),
            None => return syntax!(SyntaxError::UnexpectedEnd)
        }
    }

    Ok(ResamplingFilter::Nearest)
}

fn expect_end(it: &mut TokenStream, command: Command) -> Result<Command, Error> {
    it.next();
    match it.peek() {
        Some(&(c, _)) => syntax!(SyntaxError::UnexpectedToken(c)),
        None => Ok(command)
    }
}