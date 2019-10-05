use crate::{command::Command, ResamplingFilter, Output, Entries, syntax, error::{Error, SyntaxError}};
use super::{Token, TokenStream, Flag};
use std::{path::PathBuf, iter::Iterator};
use icon_baker::Icon;

pub fn parse<
    I: Icon,
    F: 'static + FnMut(Entries<<I as Icon>::Key>, Output) -> Command,
    G: FnMut(u32) -> Option<<I as Icon>::Key>
>(
    constructor: F,
    converter: G
) -> impl Fn(&mut TokenStream, usize) -> Result<Command, Error> {
    |it, n_entries| {
        let mut entries: Entries<<I as Icon>::Key> = Vec::with_capacity(n_entries);
        it.next();
    
        while let Some(&(c, token)) = it.peek() {
            match token {
                Token::Flag(Flag::Entry) => entry(keys(converter))(it, &mut entries)?,
                Token::Flag(Flag::Output) => return construct(constructor)(it, entries),
                _ => return syntax!(SyntaxError::UnexpectedToken(c))
            }
        }

        Ok(constructor(entries, Output::Stdout))
    }
}

pub fn entry<
    I: Icon,
    F: 'static + FnMut(&mut TokenStream, &mut Entries<<I as Icon>::Key>, &PathBuf) -> Result<(), Error>
>(
    mut adder: F,
) -> impl FnMut(&mut TokenStream, &mut Entries<<I as Icon>::Key>) -> Result<(), Error> {
    move |it, entries| {
        it.next();
        match it.peek() {
            Some(&(_, Token::Path(path))) => adder(it, entries, path),
            Some(&(c, _)) => syntax!(SyntaxError::UnexpectedToken(c)),
            None => syntax!(SyntaxError::UnexpectedEnd)
        }
    }
}

pub fn construct<I: Icon, F: 'static + FnMut(Entries<<I as Icon>::Key>, Output) -> Command>(
    mut cons: F
) -> impl FnMut(&mut TokenStream, Entries<<I as Icon>::Key>) -> Result<Command, Error> {
    move |it, entries| {
        it.next();
        match it.peek() {
            Some(&(_, Token::Path(path))) => expect_end(it, cons(entries, Output::Path(path.clone()))),
            Some(&(c, _)) => syntax!(SyntaxError::UnexpectedToken(c)),
            None => syntax!(SyntaxError::UnexpectedEnd)
        }
    }
}

pub fn keys<I: Icon, F: FnMut(u32) -> Option<<I as Icon>::Key>>(
    mut f: F
) -> impl FnMut(&mut TokenStream, &mut Entries<<I as Icon>::Key>, &PathBuf) -> Result<(), Error> {
    move |it, entries, path| {
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
            if let Some(key) = f(size) {
                entries.push((key, path.clone(), filter));
            } else {
                return Err(Error::InvalidDimensions(size));
            }
        }
    
        Ok(())
    }
}

pub fn filter(it: &mut TokenStream) -> Result<ResamplingFilter, Error> {
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