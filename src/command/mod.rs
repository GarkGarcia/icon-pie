use crate::{error::{Error, FileError}, Entries, Output, TITLE, VERSION, USAGE, COMMANDS, EXAMPLES};
use std::{io::{self, stdout}, fs, path::PathBuf, collections::HashMap};
use icon_baker::{ico::Ico, icns::Icns, favicon::Favicon, Icon, SourceImage};
use crossterm::{style, Color};

pub enum Command {
    Help,
    Version,
    Ico(Entries<<Ico as Icon>::Key>, Output),
    Icns(Entries<<Icns as Icon>::Key>, Output),
    Favicon(Entries<<Favicon as Icon>::Key>, Output)
}

impl Command {
    pub fn eval(self) -> Result<(), Error> {
        match self {
            Command::Icns(entries, output) => icon::<Icns>(entries, output)?,
            Command::Ico(entries, output) => icon::<Ico>(entries, output)?,
            Command::Favicon(entries, output) => icon::<Favicon>(entries, output)?,
            Command::Help => help(),
            Command::Version => version()
        }

        Ok(())
    }
}

fn icon<I: Icon>(entries: Entries<<I as Icon>::Key>, output: Output) -> Result<(), Error> {
    let mut icon = I::new();
    let mut source_map = HashMap::with_capacity(entries.len());

    for (key, path, filter) in entries {
        let src = source_map.entry(path.clone())
            .or_insert(source_image(&path)?);

        if let Err(err) = icon.add_entry(|src, size| filter.call(src, size), src, key) {
            return Err(Error::from_baker(err, path.clone()));
        }
    }

    match &output {
        Output::Path(path) => {
            let mut file = fs::File::create(path.clone())
                .map_err(|err| Error::Output(err, output.clone()))?;

            icon.write(&mut file)
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
fn source_image(path: &PathBuf) -> Result<SourceImage, Error> {
    match SourceImage::open(path) {
        Some(src) => Ok(src),
        None => Err(
            Error::File(FileError(io::Error::from(io::ErrorKind::NotFound), path.clone()))
        )
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