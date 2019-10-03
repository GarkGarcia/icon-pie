use crate::{error::Error, Entries, Output, TITLE, VERSION, USAGE, COMMANDS, EXAMPLES};
use std::{io::{self, stdout}, fs, path::PathBuf, collections::HashMap};
use icon_baker::{ico::Ico, icns::Icns, favicon::Favicon, Icon, SourceImage};
use crossterm::{style, Color};

pub enum Command {
    Help,
    Version,
    Ico(Entries, Output),
    Icns(Entries, Output),
    Favicon(Entries, Output)
}

macro_rules! io {
    ($err:expr) => {
        Error::Io($err, Output::Stdout)
    };
    ($err:expr, $path:expr) => {
        Error::Io($err, Output::Path($path))
    };
}

impl Command {
    pub fn eval(&self) -> Result<(), Error> {
        match self {
            Command::Icns(entries, output) => write(&mut parse_icon::<Ico>(entries)?, output)?,
            Command::Ico(entries, output) => write(&mut parse_icon::<Icns>(entries)?, output)?,
            Command::Favicon(entries, output) => write(&mut parse_icon::<Favicon>(entries)?, output)?,
            Command::Help => help(),
            Command::Version => version()
        }

        Ok(())
    }
}

fn parse_icon<I: Icon>(entries: &Entries) -> Result<I, Error> {
    let mut icon = I::new();
    let mut source_map = HashMap::with_capacity(entries.len());

    for &(size, path, filter) in entries {
        let src = source_map.entry(path)
            .or_insert(source_image(&path)?);

        let key = match I::Key::new(size) {
            Some(key) => key,
            None => return Err(Error::InvalidDimensions(size))
        };

        icon.add_entry(|src, size| filter.call(src, size), src, key)?;
    }

    Ok(icon)
}

#[inline]
fn source_image(path: &PathBuf) -> Result<SourceImage, Error> {
    match SourceImage::open(path) {
        Some(src) => Ok(src),
        None => Err(io!(io::Error::from(io::ErrorKind::NotFound), path.clone()))
    }
}

fn write<I: Icon>(icon: &mut I, output: &Output) -> Result<(), Error> {
    match output {
        Output::Path(path) => match fs::File::create(path.clone()) {
            Ok(mut file) => icon.write(&mut file)
                .map_err(|err| io!(err, path.clone())),

            Err(err) => Err(io!(err, path.clone()))
        },
        Output::Stdout => icon.write(&mut stdout())
            .map_err(|err| io!(err))
    }
}

#[inline]
fn help() {
    println!(
        "{}\n{}{}",
        style(TITLE).with(Color::Green),
        style("v").with(Color::Green),
        style(VERSION).with(Color::Green)
    );

    println!(
        "\n{}\n   {}\n\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}",
        style("Usage:").with(Color::Blue),
        style(USAGE).with(Color::Green),
        style("   -e <options>          ").with(Color::Green),
        COMMANDS[0],
        style("   -r <filter>           ").with(Color::Green),
        COMMANDS[1],
        style("   -ico [<output path>]  ").with(Color::Green),
        COMMANDS[2],
        style("   -icns [<output path>] ").with(Color::Green),
        COMMANDS[3],
        style("   -png [<output path>]  ").with(Color::Green),
        COMMANDS[4],
        style("   -h, --help            ").with(Color::Green),
        COMMANDS[5],
        style("   -v, --version         ").with(Color::Green),
        COMMANDS[6]
    );

    println!(
        "\n{}\n   {}\n   {}\n   {}\n",
        style("Examples:").with(Color::Blue),
        style(EXAMPLES[0]).with(Color::Green),
        style(EXAMPLES[1]).with(Color::Green),
        style(EXAMPLES[2]).with(Color::Green)
    );
}

#[inline]
fn version() {
    println!("icon-pie v{}", VERSION);
}