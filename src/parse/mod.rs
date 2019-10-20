use crate::{command::{Command, FaviconConfig}, syntax, error::{Error, SyntaxError}, Output};
use std::{convert::TryFrom, iter::{Iterator, Peekable, Enumerate}, slice::Iter};
use icon_baker::{Icon, ico::Ico, icns::Icns, favicon::Favicon};

mod combinators;
mod token;

use token::{Flag, Token, Cmd};
use combinators::{entries, expect_end, output};

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
    let mut web_app = false;
    let mut apple_touch = false;
    
    let entries = entries::<Favicon, _>(
        <Favicon as Icon>::Key::try_from,
        it, n_entries
    )?;

    while let Some(&(c, Token::Flag(flag))) = it.peek() {
        match flag {
            Flag::AppleTouch => {
                if !apple_touch {
                    apple_touch = true;
                    it.next();
                } else {
                    return syntax!(SyntaxError::UnexpectedToken(c));
                }
            },
            Flag::WebApp => {
                if !web_app {
                    web_app = true;
                    it.next();
                } else {
                    return syntax!(SyntaxError::UnexpectedToken(c));
                }
            },
            _ => break
        }
    }

    let config = FaviconConfig::new(web_app, apple_touch);

    match it.peek() {
        Some((_, Token::Flag(Flag::Output))) => {
            output::<Favicon, _>(move |entries, out| Command::Favicon(entries, config, out), it, entries)
        },
        None => Ok(Command::Favicon(entries, config, Output::Stdout)),
        Some(&(c, _)) => syntax!(SyntaxError::UnexpectedToken(c))
    }
}

#[inline]
fn icns(it: &mut TokenStream, n_entries: usize) -> Result<Command, Error> {
    let entries = entries::<Icns, _>(
        <Icns as Icon>::Key::try_from,
        it, n_entries
    )?;

    match it.peek() {
        Some((_, Token::Flag(Flag::Output))) => output::<Icns, _>(Command::Icns, it, entries),
        None => Ok(Command::Icns(entries, Output::Stdout)),
        Some(&(c, _)) => syntax!(SyntaxError::UnexpectedToken(c))
    }
}

#[inline]
fn ico(it: &mut TokenStream, n_entries: usize) -> Result<Command, Error> {
    let entries = entries::<Ico, _>(
        <Ico as Icon>::Key::try_from,
        it, n_entries
    )?;

    match it.peek() {
        Some((_, Token::Flag(Flag::Output))) => output::<Ico, _>(Command::Ico, it, entries),
        None => Ok(Command::Ico(entries, Output::Stdout)),
        Some(&(c, _)) => syntax!(SyntaxError::UnexpectedToken(c))
    }
}