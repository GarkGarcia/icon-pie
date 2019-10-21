use crate::{error::{Error, FileError}, Entries, Output};
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

const VERSION: &str = "0.1.4-beta";
const TITLE: &str = r"
 _____               ______ _      
|_   _|              | ___ (_)     
  | |  ___ ___  _ __ | |_/ /_  ___ 
  | | / __/ _ \| '_ \|  __/| |/ _ \
 _| || (_| (_) | | | | |   | |  __/
 \___/\___\___/|_| |_\_|   |_|\___|";
const USAGE: [&str;5] = [
    "icon-pie icns ((-e | --entry) <file path> <size>... [(-r | --resample) (nearest | linear | cubic)])... [(-o | --output) <path>]",
    "icon-pie ico ((-e | --entry) <file path> <size>... [(-r | --resample) (nearest | linear | cubic)])... [(-o | --output) <path>]",
    "icon-pie favicon ((-e | --entry) <file path> <size>... [(-r | --resample) (nearest | linear | cubic)])... [--apple-touch] [--web-app] [(-o | --output) <path>]",
    "icon-pie (-h | --help)",
    "icon-pie (-v | --version)"
];

const OPTIONS: [&str;7] = [
    "Specify an entry's source image and target sizes.",
    "Specify a re-sampling filter: `nearest`, `linear` or `cubic`. If no filter is specified the app defaults to `nearest`.",
    "Specify an output path. This is optional. If absent the output is directed to `stdout`.",
    "Favicon specific option. Confire the output to include link tags for apple-touch icons in the HTML helper.",
    "Favicon specific option. Confire the output to include a `.webmanifest` helper for PWA icons.",
    "Help.",
    "Display version information.",
];

const EXAMPLES: [&str;3] = [
    "$ icon-pie ico -e big.svg 32 64 128 -o icon.ico",
    "$ icon-pie icns -e small.png 32 64 -e big.svg 128 -o icon.icns",
    "$ icon-pie favicon -e small.png 64 128 -r linear -o ./favicon/"
];

impl Command {
    pub fn eval(self) -> Result<(), Error> {
        match self {
            Command::Icns(entries, out) => write(&mut icon::<Icns>(entries)?, out)?,
            Command::Ico(entries, out) => write(&mut icon::<Ico>(entries)?, out)?,
            Command::Favicon(entries, config, out) => {
                write(
                    icon::<Favicon>(entries)?.apple_touch(config.apple_touch).web_app(config.web_app),
                    out
                )?
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

fn write<I: Icon>(icon: &mut I, output: Output) -> Result<(), Error> {
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
        "\n{}\n   {}\n   {}\n   {}\n   {}\n   {}",
        style("Usage:").with(Color::Blue),
        style(USAGE[0]).with(Color::Green),
        style(USAGE[1]).with(Color::Green),
        style(USAGE[2]).with(Color::Green),
        style(USAGE[3]).with(Color::Green),
        style(USAGE[4]).with(Color::Green),
    );

    println!(
        "\n{}\n   {}{}\n   {}{}\n   {}{}\n   {}{}\n   {}{}\n   {}{}\n   {}{}",
        style("Options:").with(Color::Blue),
        style("-e FILE (SIZE)..., --entry FILE (SIZE)... ").with(Color::Green),
        OPTIONS[0],
        style("-r FILTER, --resample FILTER              ").with(Color::Green),
        OPTIONS[1],
        style("-o PATH, --output PATH                    ").with(Color::Green),
        OPTIONS[2],
        style("--apple-touch                             ").with(Color::Green),
        OPTIONS[3],
        style("--web-app                                 ").with(Color::Green),
        OPTIONS[4],
        style("-h, --help                                ").with(Color::Green),
        OPTIONS[5],
        style("-v, --version                             ").with(Color::Green),
        OPTIONS[6]
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