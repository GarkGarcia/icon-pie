use crate::{command::Command, syntax, error::{Error, SyntaxError}};
use std::{iter::{Iterator, Peekable, Enumerate}, slice::Iter};
use icon_baker::{Icon, ico::{Ico, IcoKey}, icns::Icns, favicon::Favicon};

mod combinators;
mod token;

use token::{Flag, Token, Cmd};
use combinators::{icon, expect_end};

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
        Some(&(_, Token::Command(Cmd::Ico))) => ico(&mut it, n_entries),
        Some(&(_, Token::Command(Cmd::Icns))) => icns(&mut it, n_entries),
        Some(&(_, Token::Command(Cmd::Favicon))) => favicon(&mut it, n_entries),
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

#[inline]
fn favicon(it: &mut TokenStream, n_entries: usize) -> Result<Command, Error> {
    icon::<Favicon, _, _>(
        Command::Favicon,
        <Favicon as Icon>::Key::new,
        it, n_entries
    )
}

#[inline]
fn icns(it: &mut TokenStream, n_entries: usize) -> Result<Command, Error> {
    icon::<Icns, _, _>(
        Command::Icns,
        <Icns as Icon>::Key::from,
        it, n_entries
    )
}

#[inline]
fn ico(it: &mut TokenStream, n_entries: usize) -> Result<Command, Error> {
    icon::<Ico, _, _>(
        Command::Ico,
        |size| if size < 256 { IcoKey::new(size as u8) } else { None },
        it, n_entries
    )
}