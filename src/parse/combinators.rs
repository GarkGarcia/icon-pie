use crate::{command::Command, ResamplingFilter, Output, Entries, syntax, error::{Error, SyntaxError}};
use super::{Token, TokenStream, Flag};
use std::{io, path::PathBuf, iter::Iterator};
use icon_baker::Icon;

pub fn entries<I: Icon,F: FnMut(u32) -> io::Result<<I as Icon>::Key>>(
    mut converter: F,
    it: &mut TokenStream,
    n_entries: usize
) -> Result<Entries<<I as Icon>::Key>, Error> {
    let mut entries: Entries<<I as Icon>::Key> = Vec::with_capacity(n_entries);
    it.next();

    while let Some(&(_, Token::Flag(Flag::Entry))) = it.peek() {
        entry::<I, _>(|size| converter(size), it, &mut entries)?;
    }

    Ok(entries)
}

pub fn entry<I: Icon, F: FnMut(u32) -> io::Result<<I as Icon>::Key>>(
    adder: F,
    it: &mut TokenStream,
    entries: &mut Entries<<I as Icon>::Key>
) -> Result<(), Error> {
    it.next();
    match it.peek() {
        Some(&(_, Token::Path(path))) => keys::<I, _>(adder, it, entries, path),
        Some(&(c, _)) => syntax!(SyntaxError::UnexpectedToken(c)),
        None => syntax!(SyntaxError::UnexpectedEnd)
    }
}

fn keys<I: Icon, F: FnMut(u32) -> io::Result<<I as Icon>::Key>>(
    mut converter: F,
    it: &mut TokenStream,
    entries: &mut Entries<<I as Icon>::Key>,
    path: &PathBuf
) -> Result<(), Error> {
    // TODO Preallocate this Vec
    let mut sizes = Vec::with_capacity(0);

    it.next();
    match it.peek() {
        Some(&(_, Token::Size(_))) => {
            while let Some(&(_, Token::Size(size))) = it.peek() {
                it.next();
                sizes.push(*size);
            }
        },
        Some(&(c, _)) => return syntax!(SyntaxError::UnexpectedToken(c)),
        None => return syntax!(SyntaxError::UnexpectedEnd)
    }

    let filter = filter(it)?;

    for size in sizes {
        if let Ok(key) = converter(size) {
            entries.push((key, path.clone(), filter));
        } else {
            return Err(Error::InvalidDimensions(size));
        }
    }

    Ok(())
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

#[inline]
pub fn expect_end(it: &mut TokenStream, command: Command) -> Result<Command, Error> {
    it.next();
    match it.peek() {
        Some(&(c, _)) => syntax!(SyntaxError::UnexpectedToken(c)),
        None => Ok(command)
    }
}

pub fn output<I: Icon, F: 'static + FnMut(Entries<<I as Icon>::Key>, Output) -> Command>(
    mut constructor: F,
    it: &mut TokenStream,
    entries: Entries<<I as Icon>::Key>
) -> Result<Command, Error> {
    it.next();
    match it.peek() {
        Some(&(_, Token::Path(path))) => expect_end(it, constructor(entries, Output::Path(path.clone()))),
        Some(&(c, _)) => syntax!(SyntaxError::UnexpectedToken(c)),
        None => syntax!(SyntaxError::UnexpectedEnd)
    }
}