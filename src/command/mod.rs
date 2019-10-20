use crate::{error::{Error, FileError}, Entries, Output, TITLE, VERSION, USAGE, COMMANDS, EXAMPLES};
use std::{io::stdout, collections::HashMap};
use icon_baker::{ico::Ico, icns::Icns, favicon::Favicon, Icon, SourceImage};
use crossterm::{style, Color};

pub enum Command {
    Help,
    Version,
    Ico(Entries<<Ico as Icon>::Key>, Output),
    Icns(Entries<<Icns as Icon>::Key>, Output),
    Favicon(Entries<<Favicon as Icon>::Key>, FaviconConfig, Output)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FaviconConfig {
    apple_touch: bool,
    web_app: bool
}

impl Command {
    pub fn eval(self) -> Result<(), Error> {
        match self {
            Command::Icns(entries, out) => write(&mut icon::<Icns>(entries)?, &out)?,
            Command::Ico(entries, out) => write(&mut icon::<Ico>(entries)?, &out)?,
            Command::Favicon(entries, config, cmd) => {
                write(
                    icon::<Favicon>(entries)?.apple_touch(config.apple_touch).web_app(config.web_app),
                    &cmd
                )?;
            },
            Command::Help => help(),
            Command::Version => version()
        }

        Ok(())
    }
}

impl FaviconConfig {
    pub fn new(web_app: bool, apple_touch: bool) -> Self {
        FaviconConfig { web_app, apple_touch }
    }
}

/// Trys to create an `I` from an `Entries<I::Key>`.
fn icon<I: Icon>(entries: Entries<I::Key>) -> Result<I, Error> {
    let mut icon = I::with_capacity(entries.len());
    let mut source_map = HashMap::with_capacity(entries.len());

    for (key, path, filter) in entries {
        let src = source_map.entry(path.clone())
            .or_insert(SourceImage::open(&path).map_err(|err| FileError(err, path.clone()))?);

        if let Err(err) = icon.add_entry(|src, size| filter.call(src, size), src, key) {
            return Err(Error::from_baker(err, path.clone()));
        }
    }

    Ok(icon)
}

fn write<I: Icon>(
    icon: &mut I,
    output: &Output
) -> Result<(), Error> {
    match &output {
        Output::Path(path) => {
            icon.save(path)
                .map_err(|err| Error::Output(err, output.clone()))?;

            println!(
                "{} Icon saved at {}.",
                style("[Success]").with(Color::Green),
                style(path.display()).with(Color::Blue)
            );
            
            Ok(())
        },
        Output::Stdout => icon.write(&mut stdout())
            .map_err(|err| Error::Output(err, Output::Stdout))
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