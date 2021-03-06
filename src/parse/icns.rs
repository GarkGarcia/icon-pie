use crate::{command::Command,Entries, Output, syntax, error::{Error, SyntaxError}};
use super::{filter, expect_end, add_entry, Token, TokenStream, Flag};
use std::{path::PathBuf, iter::Iterator};
use icon_baker::{Icon, icns::Icns};

pub fn parse(it: &mut TokenStream, n_entries: usize) -> Result<Command, Error> {
    let mut entries = Vec::with_capacity(n_entries);
    it.next();

    while let Some(&(c, token)) = it.peek() {
        match token {
            Token::Flag(Flag::Entry) => add_entry::<Icns, _>(it, &mut entries, entry_adder)?,
            Token::Flag(Flag::Output) => return command(it, entries),
            _ => return syntax!(SyntaxError::UnexpectedToken(c))
        }
    }

    Ok(Command::Icns(entries, Output::Stdout))
}

fn command(it: &mut TokenStream, entries: Entries<<Icns as Icon>::Key>) -> Result<Command, Error> {
    it.next();
    match it.peek() {
        Some(&(_, Token::Path(path))) => expect_end(it, Command::Icns(entries, Output::Path(path.clone()))),
        Some(&(c, _)) => syntax!(SyntaxError::UnexpectedToken(c)),
        None => syntax!(SyntaxError::UnexpectedEnd)
    }
}

fn entry_adder(it: &mut TokenStream, entries: &mut Entries<<Icns as Icon>::Key>, path: &PathBuf) -> Result<(), Error> {
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
        if let Some(key) = <Icns as Icon>::Key::from(size) {
            entries.push((key, path.clone(), filter));
        } else {
            return Err(Error::InvalidDimensions(size));
        }
    }

    Ok(())
}