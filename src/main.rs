extern crate icon_baker;
extern crate crossterm;

mod parse;
mod eval;
mod error;

use std::{env, io, path::{PathBuf}};
use icon_baker::Size;
use crossterm::{style, Color};

pub enum Command {
    Help,
    Version,
    Icon(Entries, IconType, Output)
}

#[derive(Clone, Debug)]
pub enum Output {
    Path(PathBuf),
    Stdout
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IconType {
    Ico,
    Icns,
    PngSequence
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ResamplingFilter {
    Nearest,
    Linear,
    Cubic
}

pub type Entries = Vec<(Size, PathBuf, ResamplingFilter)>;

#[macro_export]
macro_rules! syntax {
    ($err:expr) => { Err(Error::Syntax($err)) };
}

const VERSION: &str = "0.1.1-beta";
const TITLE: &str = r"
 _____               ______ _      
|_   _|              | ___ (_)     
  | |  ___ ___  _ __ | |_/ /_  ___ 
  | | / __/ _ \| '_ \|  __/| |/ _ \
 _| || (_| (_) | | | | |   | |  __/
 \___/\___\___/|_| |_\_|   |_|\___|";
const USAGE: &str = "icon-pie ((-e <file path> <size>... [-r (nearest | linear | cubic)])... (-ico | -icns | -png) [<output path>]) | -h | --help | -v | --version";
const EXAMPLES: [&str;3] = [
    "icon-pie -e small.svg 16 20 24 -e big.png 32 64 -ico output.ico",
    "icon-pie -e image.png 32 64 48 -r linear -png output.tar",
    "echo Here's an ICNS file: ${ icon-pie -e image.jpg 16 32 64 -r cubic -icns | hexdump }"
];

const COMMANDS: [&str;7] = [
    "Specify an entrie's options.",
    "Specify a resampling filter: 'nearest', 'linear' or 'cubic'. If no filter is specified the app defaults to 'nearest'.",
    "Outputs to a .ico file. If no output path is specified the app outputs to stdout.",
    "Outputs to a .icns file. If no output path is specified the app outputs to stdout.",
    "Outputs a .png sequence as a .tar file. If no output path is specified the app outputs to stdout.",
    "Help.",
    "Display version information.",
];

fn main() -> io::Result<()> {
    match parse::args() {
        Ok(cmd)  => command(cmd),
        Err(err) => Err(err.exit_with())
    }
}

fn command(cmd: Command) -> io::Result<()> {
    match cmd {
        Command::Icon(entries, icon_type, output) => icon(&entries, icon_type, output)?,
        Command::Help    => help(),
        Command::Version => version()
    }

    Ok(())
}

fn icon(entries: &Entries, icon_type: IconType, output: Output) -> io::Result<()> {
    eval::icon(&entries, icon_type, &output)
        .map_err(|err| err.exit_with())?;

    if let Output::Path(path) = output {
        println!(
            "{} Output saved at {}.",
            style("[Success]").with(Color::Green),
            style(path.display()).with(Color::Blue)
        );
    }

    Ok(())
}

#[inline]
fn help() {
    println!(
        "{}\nV {}",
        style(TITLE).with(Color::Green),
        style(VERSION).with(Color::Green)
    );

    println!("\n{} {}\n\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}",
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

    println!("\n{}\n   {}\n   {}\n   {}\n",
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

fn args() -> Vec<String> {
    let output: Vec<String> = env::args_os()
        .map(|os_str| String::from(os_str.to_string_lossy()))
        .collect();

    if output.len() > 0 {
        if let parse::Token::Path(_) = parse::Token::from(output[0].as_ref()) {
            return Vec::from(&output[1..]);
        }
    }

    output
}